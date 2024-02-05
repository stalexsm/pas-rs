use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    check_access, check_is_admin,
    services::{Items, Select},
    AppError, CurrentUser,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    name: String,
    organization_id: Option<i64>,
    measure_unit_id: i64,
}

pub async fn create_product(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика создания продукта

    if !check_access(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let organization_id = if check_is_admin(current_user.role) {
            body.organization_id
        } else {
            current_user.organization_id
        };

        match organization_id {
            Some(organization_id) => {
                let row: (i64,) = sqlx::query_as(
                    "INSERT
INTO products (name, measure_unit_id, organization_id) VALUES
                    ($1, $2, $3) RETURNING id",
                )
                .bind(body.name)
                .bind(body.measure_unit_id)
                .bind(organization_id)
                .fetch_one(&pool)
                .await?;

                Ok(row.0)
            }
            _ => Err(AppError(
                StatusCode::BAD_REQUEST,
                anyhow::anyhow!("Невозможно создать запись без организации!"),
            )),
        }
    }
}

pub async fn edit_product(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика редактирования продукта

    if !check_access(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let organization_id = if check_is_admin(current_user.role) {
            body.organization_id
        } else {
            current_user.organization_id
        };

        match organization_id {
            Some(organization_id) => {
                let _ = sqlx::query(
                    "UPDATE products
SET name=$1, measure_unit_id=$2, organization_id=$3, updated_at=NOW()
                    WHERE id = $4",
                )
                .bind(body.name)
                .bind(body.measure_unit_id)
                .bind(organization_id)
                .bind(id)
                .execute(&pool)
                .await?;

                Ok(id)
            }
            _ => Err(AppError(
                StatusCode::BAD_REQUEST,
                anyhow::anyhow!("Невозможно отредактировать запись без организации!"),
            )),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Q {
    #[serde(default = "page")]
    pub page: i64,
    #[serde(default = "per_page")]
    pub per_page: i64,
}

fn per_page() -> i64 {
    15
}

fn page() -> i64 {
    1
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,

    organization: Select,
    measure_unit: Select,
}

pub async fn get_products(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Items<Item>, anyhow::Error> {
    // Бизнес логика получения списка продуктв

    let rows = sqlx::query!(
        "SELECT
            p.id,
            p.name,
            p.created_at,
            JSONB_BUILD_OBJECT(
                'id', mu.id,
                'name', mu.name
            ) AS measure_unit,
            JSONB_BUILD_OBJECT(
                'id', o.id,
                'name', o.name
            ) AS organization
        FROM products AS p
        LEFT JOIN measure_units AS mu on mu.id = p.measure_unit_id
        LEFT JOIN organizations AS o ON o.id = p.organization_id
        WHERE
            CASE
                WHEN $1::bigint IS NOT NULL AND $2 not in ('Admin', 'Developer') THEN
                    p.organization_id = $1
                ELSE TRUE
            END
        ORDER BY p.id DESC
        OFFSET $3 LIMIT $4;",
        current_user.organization_id,
        current_user.role.to_string(),
        (q.page - 1) * q.per_page,
        q.per_page,
    )
    .map(|row| Item {
        id: row.id,
        name: row.name,
        created_at: row.created_at,
        organization: row.organization.into(),
        measure_unit: row.measure_unit.into(),
    })
    .fetch_all(&pool)
    .await?;

    // Подсчет данных для пагинации
    let cnt: i64 = sqlx::query_scalar("SELECT COUNT(id) FROM products;")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let cnt = (cnt as f64 / q.per_page as f64).ceil() as i64;

    Ok(Items { items: rows, cnt })
}

pub async fn detail_product(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<Item, AppError> {
    // Бизнес логика получения продуктв

    if !check_access(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let row = sqlx::query!(
            "SELECT
                p.id,
                p.name,
                p.created_at,
                JSONB_BUILD_OBJECT(
                    'id', mu.id,
                    'name', mu.name
                ) AS measure_unit,
                JSONB_BUILD_OBJECT(
                    'id', o.id,
                    'name', o.name
                ) AS organization
            FROM products AS p
            LEFT JOIN measure_units AS mu on mu.id = p.measure_unit_id
            LEFT JOIN organizations AS o ON o.id = p.organization_id
        WHERE p.id = $1;",
            id,
        )
        .fetch_optional(&pool)
        .await?;

        match row {
            // Собираем в нужный вид
            Some(row) => Ok(Item {
                id: row.id,
                name: row.name,
                created_at: row.created_at,
                organization: row.organization.into(),
                measure_unit: row.measure_unit.into(),
            }),
            None => Err(AppError(
                StatusCode::FORBIDDEN,
                anyhow::anyhow!("Такой записи не существует"),
            )),
        }
    }
}

pub async fn delete_product(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    // Бизнес логика удаления продуктв

    if !check_access(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let _ = sqlx::query(
            "delete
        from products
        where id = $1;",
        )
        .bind(id)
        .execute(&pool)
        .await?;

        Ok(())
    }
}
