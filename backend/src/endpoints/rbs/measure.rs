use crate::services::rbs::measure::{self as serv, Item, RequestBody, Q};
use crate::services::Items;
use crate::{AppError, CurrentUser};
use axum::extract::{Path, Query};
use axum::Extension;
use axum::{extract::State, Json};
use sqlx::PgPool;
use std::collections::HashMap;

pub async fn create_measure(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RequestBody>,
) -> Result<Json<HashMap<String, i64>>, AppError> {
    // Метод создания единицы измерения

    let insert_id = serv::create_measure(State(pool), Extension(current_user), Json(body)).await?;

    Ok(Json(HashMap::from([("id".to_string(), insert_id)])))
}

pub async fn edit_measure(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBody>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // Метод редактирования единицы измерения

    let _ = serv::edit_measure(State(pool), Extension(current_user), Path(id), Json(body)).await?;

    Ok(Json(HashMap::from([(
        "detail".to_string(),
        "OK".to_string(),
    )])))
}

pub async fn get_measures(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Json<Items<Item>>, AppError> {
    // Метод получения списка единиц измерения

    let items = serv::get_measures(State(pool), Extension(current_user), Query(q)).await?;

    Ok(Json(items))
}

pub async fn detail_measure(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<Json<Item>, AppError> {
    // Метод получения единицы измерения

    let item = serv::detail_measure(State(pool), Extension(current_user), Path(id)).await?;

    Ok(Json(item))
}

pub async fn delete_measure(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<Json<HashMap<String, String>>, AppError> {
    // Метод удаления единицы измерения

    serv::delete_measure(State(pool), Extension(current_user), Path(id)).await?;

    Ok(Json(HashMap::from([(
        "detail".to_string(),
        "OK".to_string(),
    )])))
}
