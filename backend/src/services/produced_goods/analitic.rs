use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension,
};

use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{check_access, AppError, CurrentUser};
use rust_xlsxwriter::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub measure: String,
    pub fio: String,
    pub cnt: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Q {
    pub date_one: chrono::NaiveDate,
    pub date_two: chrono::NaiveDate,

    pub product: Option<String>,
    pub user: Option<String>,
}

pub async fn get_analitics(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Vec<Item>, AppError> {
    // Бизнес логика получения продуктв

    if !check_access(current_user.role) {
        Err(AppError(
            StatusCode::NOT_FOUND,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let rows = sqlx::query!(
            "SELECT
              p.id as id,
              p.name as name,
              mu.name as measure,
              u.fio as fio,
              SUM(pg.cnt + COALESCE(pa.adjustment_cnt::bigint, 0))::bigint AS cnt
            FROM
              products AS p
              JOIN measure_units as mu ON mu.id = p.measure_unit_id
              JOIN produced_goods as pg ON p.id = pg.product_id
              JOIN users as u ON u.id = pg.user_id
              LEFT JOIN (
                SELECT
                  produced_good_id,
                  SUM(cnt::bigint) AS adjustment_cnt
                FROM
                  produced_good_adjustments
                GROUP BY
                  produced_good_id
              ) pa ON pa.produced_good_id = pg.id
            WHERE pg.created_at::date between $1 AND $2
            AND CASE
                WHEN $3::text IS NOT NULL THEN u.id = ANY((string_to_array($3::text, ','))::bigint[])
                WHEN $4::VARCHAR IS NOT NULL THEN p.name ILIKE '%'||$4||'%'
                WHEN $5::bigint IS NOT NULL AND $6 = 'Director' THEN pg.organization_id = $5
                ELSE TRUE
              END
            GROUP BY
              p.id,
              u.fio,
              measure
            ORDER BY
              cnt desc,
              p.id desc;",
            q.date_one,
            q.date_two,
            q.user.map_or(None, |u| {
                if u.is_empty() {
                    None
                } else {
                    Some(
                        u.split(';')
                            .filter(|s| !s.is_empty())
                            .filter_map(|s| s.parse::<i64>().ok())
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                            .join(","),
                    )
                }
            }),
            q.product,
            current_user.organization_id,
            current_user.role.to_string(),
        )
        .map(|row| Item {
            id: row.id,
            name: row.name,
            measure: row.measure,
            fio: row.fio,
            cnt: row.cnt.map_or(0, |cnt| cnt),
        })
        .fetch_all(&pool)
        .await?;

        Ok(rows)
    }
}

pub async fn generate_excel(
    items: Vec<Item>,
    date_one: chrono::NaiveDate,
    date_two: chrono::NaiveDate,
) -> Result<Vec<u8>, AppError> {
    let mut wookbook = Workbook::new();

    // formats
    let right_fmt = Format::new()
        .set_align(FormatAlign::Right)
        .set_border(FormatBorder::Thin);

    // Add a worksheet to the workbook.
    let worksheet = wookbook.add_worksheet();

    // Set the column weight
    worksheet.set_column_width(0, 8)?;
    worksheet.set_column_width(1, 25)?;
    worksheet.set_column_width(2, 25)?;
    worksheet.set_column_width(3, 15)?;
    worksheet.set_column_width(4, 25)?;

    let _ = worksheet.merge_range(
        0,
        0,
        0,
        4,
        &format!(
            "Отчет по производству товаров за период: {} - {}",
            date_one.format("%d.%m.%Y"),
            date_two.format("%d.%m.%Y")
        ),
        &Format::new()
            .set_bold()
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin),
    );
    // Высота
    let _ = worksheet.set_row_height(0, 30);
    let _ = worksheet.set_row_height(1, 20);

    let mut i = 0;
    ["#", "Продукт", "Пользователь", "Ед.измерения", "Кол-во"].map(|title| {
        let _ = worksheet.write_with_format(
            1,
            i,
            title,
            &Format::new()
                .set_bold()
                .set_align(FormatAlign::Center)
                .set_background_color(Color::RGB(0xC6C6C6))
                .set_border(FormatBorder::Thin),
        );
        i += 1
    });

    let mut i = 2;
    items.iter().for_each(|item| {
        let _ = worksheet.write_with_format(i, 0, item.id, &right_fmt);
        let _ = worksheet.write_with_format(i, 1, item.name.clone(), &right_fmt);
        let _ = worksheet.write_with_format(i, 2, item.fio.clone(), &right_fmt);
        let _ = worksheet.write_with_format(i, 3, item.measure.clone(), &right_fmt);
        let _ = worksheet.write_with_format(
            i,
            4,
            item.cnt,
            &Format::new().set_border(FormatBorder::Thin),
        );

        i += 1;
    });

    let buffer = wookbook.save_to_buffer()?;

    Ok(buffer)
}
