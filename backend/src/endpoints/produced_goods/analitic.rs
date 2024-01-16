use crate::services::produced_goods::analitic::{self as serv, Item, Q};

use crate::{AppError, CurrentUser};
use axum::extract::Query;
use axum::Extension;
use axum::{extract::State, Json};
use sqlx::PgPool;

pub async fn get_analitics(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Json<Vec<Item>>, AppError> {
    // Метод получения списка единиц измерения

    let items = serv::get_analitics(State(pool), Extension(current_user), Query(q)).await?;

    Ok(Json(items))
}
