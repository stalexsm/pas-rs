use crate::{check_is_admin, services::Items, AppError, CurrentUser};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    name: String,
}

pub async fn create_organization(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика создания пользователя

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let row: (i64,) = sqlx::query_as(
            "insert
            into organizations (name) values
            ($1) returning id",
        )
        .bind(body.name)
        .fetch_one(&pool)
        .await?;

        Ok(row.0)
    }
}

pub async fn edit_organization(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика редактирования пользователя

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let _ = sqlx::query(
            "update organizations
            set name=$1, updated_at=NOW()
            where id = $2",
        )
        .bind(body.name)
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
pub struct Item {
    pub id: i64,
    pub name: String,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_organizations(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Items<Item>, AppError> {
    // Бизнес логика редактирования пользователя

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let rows = sqlx::query_as!(
            Item,
            "select
            id,
            name,
            created_at
        from organizations
        order by id desc
        offset $1 limit $2;",
            (q.page - 1) * q.per_page,
            q.per_page,
        )
        .fetch_all(&pool)
        .await?;

        // Подсчет данных для пагинации
        let cnt: i64 = sqlx::query_scalar("select count(id) from organizations;")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        let cnt = (cnt as f64 / q.per_page as f64).ceil() as i64;

        Ok(Items { items: rows, cnt })
    }
}

pub async fn detail_organization(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<Item, AppError> {
    // Бизнес логика редактирования пользователя

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let row = sqlx::query_as!(
            Item,
            "select
            id,
            name,
            created_at
        from organizations
        where id = $1;",
            id
        )
        .fetch_optional(&pool)
        .await?;

        match row {
            Some(row) => Ok(row),
            None => Err(AppError(
                StatusCode::NOT_FOUND,
                anyhow::anyhow!("Такой записи не существует!"),
            )),
        }
    }
}
