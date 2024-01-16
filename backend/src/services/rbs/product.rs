use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::{check_is_admin, services::Items, AppError, CurrentUser};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    name: String,
    measure_unit_id: i64,
}

pub async fn create_product(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика создания продукта

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let row: (i64,) = sqlx::query_as(
            "insert
            into products (name, measure_unit_id) values
            ($1, $2) returning id",
        )
        .bind(body.name)
        .bind(body.measure_unit_id)
        .fetch_one(&pool)
        .await?;

        Ok(row.0)
    }
}

pub async fn edit_product(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика редактирования продукта

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let _ = sqlx::query(
            "update products
            set name=$1, measure_unit_id=$2, updated_at=NOW()
            where id = $3",
        )
        .bind(body.name)
        .bind(body.measure_unit_id)
        .bind(id)
        .execute(&pool)
        .await?;

        Ok(id)
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
pub struct Select {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,

    measure_unit: Select,
}

pub async fn get_products(
    State(pool): State<PgPool>,
    Extension(_current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Items<Item>, anyhow::Error> {
    // Бизнес логика получения списка продуктв

    let rows = sqlx::query(
        r#"select
            products.id,
            products.name,
            products.created_at,
            measure_units.id as "measure_unit_id",
            measure_units.name as "measure_unit_name"
        from products
        inner join measure_units on measure_units.id = products.measure_unit_id
        order by products.id desc
        offset $1 limit $2;"#,
    )
    .bind((q.page - 1) * q.per_page)
    .bind(q.per_page)
    .map(|row: PgRow| Item {
        id: row.get(0),
        name: row.get(1),
        created_at: row.get(2),
        measure_unit: Select {
            id: row.get(3),
            name: row.get(4),
        },
    })
    .fetch_all(&pool)
    .await?;

    // Подсчет данных для пагинации
    let cnt: i64 = sqlx::query_scalar("select count(id) from products;")
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

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let row = sqlx::query(
            r#"select
            products.id,
            products.name,
            products.created_at,
            measure_units.id as "measure_unit_id",
            measure_units.name as "measure_unit_name"
        from products
        inner join measure_units on measure_units.id = products.measure_unit_id
        where products.id = $1;"#,
        )
        .bind(id)
        .fetch_optional(&pool)
        .await?;

        match row {
            // Собираем в нужный вид
            Some(row) => Ok(Item {
                id: row.get(0),
                name: row.get(1),
                created_at: row.get(2),
                measure_unit: Select {
                    id: row.get(3),
                    name: row.get(4),
                },
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

    if !check_is_admin(current_user.role) {
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
