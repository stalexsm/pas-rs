use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{check_is_admin, AppError, CurrentUser, Role};

use super::Items;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    email: String,
    fio: String,
    role: String,
    blocked: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBodyPasswd {
    passwd1: String,
    passwd2: String,
}

pub async fn create_user(
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
        let mut body_blocked = false;
        if let Some(_blocked) = body.blocked {
            body_blocked = _blocked;
        }

        let hash_passwd = bcrypt::hash("password", bcrypt::DEFAULT_COST)?;
        let row: (i64,) = sqlx::query_as(
            "insert
            into users (role, email, fio, passwd, blocked) values
            ($1, $2, $3, $4, $5) returning id",
        )
        .bind(body.role)
        .bind(body.email)
        .bind(body.fio)
        .bind(hash_passwd)
        .bind(body_blocked)
        .fetch_one(&pool)
        .await?;

        Ok(row.0)
    }
}

pub async fn edit_user(
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
        let mut body_blocked = false;
        if let Some(_blocked) = body.blocked {
            body_blocked = _blocked;
        }

        let _ = sqlx::query(
            "update users
            set role=$1, fio=$2, blocked=$3, updated_at=NOW()
            where id = $4",
        )
        .bind(body.role)
        .bind(body.fio)
        .bind(body_blocked)
        .bind(id)
        .execute(&pool)
        .await?;

        Ok(id)
    }
}

pub async fn edit_passwd(
    State(pool): State<PgPool>,
    Extension(_current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBodyPasswd>,
) -> Result<i64, AppError> {
    // Бизнес логика редактирования пароля
    // todo Сделать расширенную проверку

    if !body.passwd1.eq(&body.passwd2) {
        Err(AppError(
            StatusCode::BAD_REQUEST,
            anyhow::anyhow!("Пароли не совпадают!"),
        ))
    } else {
        let hash_passwd = bcrypt::hash(body.passwd1, bcrypt::DEFAULT_COST)?;
        let _ = sqlx::query(
            "update users
            set passwd=$1, updated_at=NOW()
            where id = $2",
        )
        .bind(hash_passwd)
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
    pub role: Role,
    pub email: String,
    pub fio: Option<String>,
    pub blocked: bool,

    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_users(
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
            role,
            email,
            fio,
            blocked,
            created_at
        from users
        order by id desc
        offset $1 limit $2;",
            (q.page - 1) * q.per_page,
            q.per_page,
        )
        .fetch_all(&pool)
        .await?;

        // Подсчет данных для пагинации
        let cnt: i64 = sqlx::query_scalar("select count(id) from users;;")
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        let cnt = (cnt as f64 / q.per_page as f64).ceil() as i64;

        Ok(Items { items: rows, cnt })
    }
}

pub async fn detail_user(
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
            role,
            email,
            fio,
            blocked,
            created_at
        from users
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

pub async fn current_user(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Item, AppError> {
    // Endpoint для получения пользовательских данных по токену.

    let row = sqlx::query_as!(
        Item,
        "select
        id,
        role,
        email,
        fio,
        blocked,
        created_at
    from users
    where id = $1;",
        current_user.id
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
