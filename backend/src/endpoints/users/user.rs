use crate::services::users::user::{self as serv, Item, RequestBody, RequestBodyPasswd, Q};
use crate::services::Items;
use crate::{AppError, CurrentUser};
use axum::extract::{Path, Query};
use axum::Extension;
use axum::{extract::State, Json};
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn create_user(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RequestBody>,
) -> Result<Json<HashMap<String, i64>>, AppError> {
    // Метод создания пользователя

    let insert_id = serv::create_user(State(pool), Extension(current_user), Json(body)).await?;

    Ok(Json(HashMap::from([("id".to_string(), insert_id)])))
}

pub async fn edit_user(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBody>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // Метод редактирования пользователя

    let _ = serv::edit_user(State(pool), Extension(current_user), Path(id), Json(body)).await?;

    Ok(Json(HashMap::from([(
        "detail".to_string(),
        "OK".to_string(),
    )])))
}

pub async fn get_users(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Json<Items<Item>>, AppError> {
    // Метод получения списка пользователей

    let items = serv::get_users(State(pool), Extension(current_user), Query(q)).await?;

    Ok(Json(items))
}

pub async fn detail_user(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<Json<Item>, AppError> {
    // Метод получения пользователя

    let item = serv::detail_user(State(pool), Extension(current_user), Path(id)).await?;

    Ok(Json(item))
}

pub async fn current_user(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
) -> Result<Json<Item>, AppError> {
    // Метод получения пользователя

    let item = serv::current_user(State(pool), Extension(current_user)).await?;

    Ok(Json(item))
}

pub async fn edit_passwd(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBodyPasswd>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // Метод редактирования пользователя

    let _ = serv::edit_passwd(State(pool), Extension(current_user), Path(id), Json(body)).await?;

    Ok(Json(HashMap::from([(
        "detail".to_string(),
        "OK".to_string(),
    )])))
}
