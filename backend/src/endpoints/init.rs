use crate::services::init::{self as serv, AuthData};
use crate::{AppError, CurrentUser};
use axum::Extension;
use axum::{extract::State, Json};
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn authorization(
    State(pool): State<PgPool>,
    Json(body): Json<AuthData>,
) -> Result<Json<HashMap<String, Uuid>>, AppError> {
    // Метод авторизации

    let token = serv::authorization(State(pool), Json(body)).await?;

    Ok(Json(HashMap::from([("token".to_string(), token)])))
}

pub async fn logout(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // Метод авторизации

    serv::logout(State(pool), Extension(current_user)).await?;

    Ok(Json(HashMap::from([(
        "detail".to_string(),
        "OK".to_string(),
    )])))
}
