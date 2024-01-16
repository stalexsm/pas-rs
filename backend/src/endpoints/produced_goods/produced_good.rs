use crate::services::produced_goods::produced_good::{
    self as serv, Item, RequestBody, RequestBodyAdj, Q,
};
use crate::services::Items;
use crate::{AppError, CurrentUser};
use axum::extract::{Path, Query};
use axum::Extension;
use axum::{extract::State, Json};
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn create_produced_good(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RequestBody>,
) -> Result<Json<HashMap<String, i64>>, AppError> {
    // Метод создания единицы измерения

    let insert_id =
        serv::create_produced_good(State(pool), Extension(current_user), Json(body)).await?;

    Ok(Json(HashMap::from([("id".to_string(), insert_id)])))
}

pub async fn edit_produced_good(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBody>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // Метод редактирования единицы измерения

    let _ = serv::edit_produced_good(State(pool), Extension(current_user), Path(id), Json(body))
        .await?;

    Ok(Json(HashMap::from([(
        "detail".to_string(),
        "OK".to_string(),
    )])))
}

pub async fn get_produced_goods(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Json<Items<Item>>, AppError> {
    // Метод получения списка единиц измерения

    let items = serv::get_produced_goods(State(pool), Extension(current_user), Query(q)).await?;

    Ok(Json(items))
}

pub async fn detail_produced_good(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<Json<Item>, AppError> {
    // Метод получения единицы измерения

    let item = serv::detail_produced_good(State(pool), Extension(current_user), Path(id)).await?;

    Ok(Json(item))
}

pub async fn add_adj_produced_goods(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBodyAdj>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // Метод редактирования единицы измерения

    serv::add_adj_produced_goods(State(pool), Extension(current_user), Path(id), Json(body))
        .await?;

    Ok(Json(HashMap::from([(
        "detail".to_string(),
        "OK".to_string(),
    )])))
}
