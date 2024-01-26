use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{check_access, check_is_admin, services::Items, AppError, CurrentUser};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    organization_id: Option<i64>,
    name: String,
}

pub async fn create_measure(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика создания пользователя

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
                    "insert
                    into measure_units (name, organization_id) values
                    ($1, $2) returning id",
                )
                .bind(body.name)
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

pub async fn edit_measure(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика редактирования пользователя

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
                    "update measure_units
                    set name=$1, organization_id=$2, updated_at=NOW()
                    where id = $3",
                )
                .bind(body.name)
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
    organization_id: i64,
    pub name: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_measures(
    State(pool): State<PgPool>,
    Extension(_current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Items<Item>, anyhow::Error> {
    // Бизнес логика редактирования пользователя

    let rows = sqlx::query_as!(
        Item,
        "select
            id,
            organization_id,
            name,
            created_at
        from measure_units
        order by id desc
        offset $1 limit $2;",
        (q.page - 1) * q.per_page,
        q.per_page,
    )
    .fetch_all(&pool)
    .await?;

    // Подсчет данных для пагинации
    let cnt: i64 = sqlx::query_scalar("select count(id) from measure_units;")
        .fetch_one(&pool)
        .await
        .unwrap_or(0);

    let cnt = (cnt as f64 / q.per_page as f64).ceil() as i64;

    Ok(Items { items: rows, cnt })
}

pub async fn detail_measure(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<Item, AppError> {
    // Бизнес логика редактирования пользователя

    if !check_access(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let row = sqlx::query_as!(
            Item,
            "select
            id,
            organization_id,
            name,
            created_at
        from measure_units
        where id = $1;",
            id
        )
        .fetch_optional(&pool)
        .await?;

        match row {
            Some(row) => Ok(row),
            None => Err(AppError(
                StatusCode::FORBIDDEN,
                anyhow::anyhow!("Такой записи не существует"),
            )),
        }
    }
}

pub async fn delete_measure(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    // Бизнес логика редактирования пользователя

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let _ = sqlx::query(
            "delete
        from measure_units
        where id = $1;",
        )
        .bind(id)
        .execute(&pool)
        .await?;

        Ok(())
    }
}
