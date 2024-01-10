use axum::{extract::State, http::StatusCode, Extension, Json};
use bcrypt;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{AppError, CurrentUser};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthData {
    email: String,
    passwd: String,
}

pub async fn authorization(
    State(pool): State<PgPool>,
    Json(body): Json<AuthData>,
) -> Result<Uuid, AppError> {
    // Бизнес логика для авторизации

    let row: Option<(i64, Option<String>)> =
        sqlx::query_as("select users.id, users.passwd from users where users.email = $1")
            .bind(&body.email)
            .fetch_optional(&pool)
            .await?;

    if let Some(row) = row {
        if let Some(passwd) = row.1 {
            if bcrypt::verify(&body.passwd, &passwd)? {
                let token = Uuid::new_v4();

                let _ = sqlx::query(
                    "insert into sessions (id, user_id, expires_at) values ($1, $2, $3);",
                )
                .bind(token)
                .bind(row.0)
                .bind(Utc::now() + Duration::days(30))
                .execute(&pool)
                .await?;

                return Ok(token);
            }
        }
    }
    Err(AppError(
        StatusCode::BAD_REQUEST,
        anyhow::anyhow!("Неверный логин или пароль!"),
    ))
}

pub async fn logout(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<(), anyhow::Error> {
    // Бизнес логика для выхода из системы

    let _ = sqlx::query(
        "delete
        from sessions
        where sessions.id = $1",
    )
    .bind(current_user.token)
    .execute(&pool)
    .await?;

    Ok(())
}
