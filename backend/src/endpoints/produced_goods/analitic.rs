use crate::services::produced_goods::analitic::{self as serv, Item, Q};

use crate::{AppError, CurrentUser};
use axum::body::Body;
use axum::extract::Query;
use axum::http::{Response, StatusCode};
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

// Content-Disposition": "attachment; filename='test.xlsx;"
pub async fn upload_report_in_excel(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Response<Body>, AppError> {
    // Метод получения списка единиц измерения

    let items = serv::get_analitics(State(pool), Extension(current_user), Query(q.clone())).await?;
    let buffer = serv::generate_excel(items, q.date_one, q.date_two).await?;

    let filename = format!(
        "period_report_{}_{}.xlsx",
        q.date_one.format("%d.%m.%Y"),
        q.date_two.format("%d.%m.%Y")
    );

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header(
            "Content-Disposition",
            format!("attachment; filename={}", filename),
        )
        .header(
            "Content-Type",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        )
        .body(Body::from(buffer))
        .unwrap();

    Ok(resp)
}
