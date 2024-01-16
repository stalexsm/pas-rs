use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension,
};

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::{check_is_admin, AppError, CurrentUser};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub measure: String,
    pub cnt: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Q {
    pub date_one: chrono::NaiveDate,
    pub date_two: chrono::NaiveDate,
}

pub async fn get_analitics(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Vec<Item>, AppError> {
    // Бизнес логика получения продуктв

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::NOT_FOUND,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let rows = sqlx::query(
            r#"SELECT
              products.id as id,
              products.name as name,
              measure_units.name as measure,
              SUM(pg.cnt + COALESCE(pa.adjustment_cnt::bigint, 0))::bigint AS cnt
            FROM
              produced_goods pg
              LEFT JOIN (
                SELECT
                  produced_good_id,
                  SUM(cnt::integer) AS adjustment_cnt
                FROM
                  produced_good_adjustments
                GROUP BY
                  produced_good_id
              ) pa ON pa.produced_good_id = pg.id
              LEFT JOIN products on products.id = pg.product_id
              LEFT JOIN measure_units on measure_units.id = products.measure_unit_id
            WHERE pg.created_at::date BETWEEN $1 AND $2
            GROUP BY
              products.id,
              products.name,
              measure_units.name
            ORDER BY cnt DESC;"#,
        )
        .bind(q.date_one)
        .bind(q.date_two)
        .map(|row: PgRow| Item {
            id: row.get(0),
            name: row.get(1),
            measure: row.get(2),
            cnt: row.get(3),
        })
        .fetch_all(&pool)
        .await?;

        Ok(rows)
    }
}
