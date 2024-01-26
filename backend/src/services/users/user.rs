use crate::{check_access, check_is_admin, services::Items, AppError, CurrentUser, Role};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use bcrypt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    email: String,
    fio: String,
    role: String,
    blocked: Option<bool>,
    organization_id: Option<i64>,
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

    if !check_access(current_user.role) {
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

        let organization_id = if check_is_admin(current_user.role) {
            body.organization_id
        } else {
            current_user.organization_id
        };

        if matches!(Role::from(body.role.clone()), Role::Director | Role::User)
            && organization_id.is_none()
        {
            return Err(AppError(
                StatusCode::BAD_REQUEST,
                anyhow::anyhow!("Невозможно создать запись без организации!"),
            ));
        }

        let row: (i64,) = sqlx::query_as(
            "insert
            into users (role, email, fio, passwd, blocked, organization_id) values
            ($1, $2, $3, $4, $5, $6) returning id",
        )
        .bind(body.role)
        .bind(body.email)
        .bind(body.fio)
        .bind(hash_passwd)
        .bind(body_blocked)
        .bind(organization_id)
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

    if !check_access(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let mut body_blocked = false;
        if let Some(_blocked) = body.blocked {
            body_blocked = _blocked;
        }

        let organization_id = if check_is_admin(current_user.role) {
            body.organization_id
        } else {
            current_user.organization_id
        };

        if matches!(Role::from(body.role.clone()), Role::Director | Role::User)
            && organization_id.is_none()
        {
            return Err(AppError(
                StatusCode::BAD_REQUEST,
                anyhow::anyhow!("Невозможно отредактировать запись без организации!"),
            ));
        }

        let _ = sqlx::query(
            "update users
            set role=$1, fio=$2, blocked=$3, organization_id=$4, updated_at=NOW()
            where id = $5",
        )
        .bind(body.role)
        .bind(body.fio)
        .bind(body_blocked)
        .bind(organization_id)
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
    pub organization_id: Option<i64>,
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

    if !check_access(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let rows = sqlx::query_as!(
            Item,
            "select
            id,
            organization_id,
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
        organization_id,
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
