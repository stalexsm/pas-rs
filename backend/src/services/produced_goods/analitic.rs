use std::io::{Seek, SeekFrom};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension,
};

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::{check_is_admin, empty_string_as_none, AppError, CurrentUser};
use rust_xlsxwriter::*;

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

    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub product: Option<String>,
    pub user: Option<String>,
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
        let mut where_additation = String::new();
        if let Some(product) = q.product {
            where_additation.push_str(&format!(" and products.name ilike '%{}%'", product));
        }

        if let Some(user) = q.user {
            where_additation.push_str(&format!(" and users.fio ilike '%{}%'", user));
        }

        let rows = sqlx::query(&format!(
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
                  LEFT JOIN users on users.id = pg.user_id
                WHERE pg.created_at::date BETWEEN $1 AND $2
                {}
                GROUP BY
                  products.id,
                  products.name,
                  measure_units.name
                ORDER BY cnt DESC;"#,
            where_additation
        ))
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

pub async fn generate_excel(
    items: Vec<Item>,
    date_one: chrono::NaiveDate,
    date_two: chrono::NaiveDate,
) -> Result<tempfile::NamedTempFile, AppError> {
    let mut wookbook = Workbook::new();

    // formats
    let bold_fmt = Format::new().set_bold().set_border(FormatBorder::Thin);
    let right_fmt = Format::new()
        .set_align(FormatAlign::Right)
        .set_border(FormatBorder::Thin);

    // Add a worksheet to the workbook.
    let worksheet = wookbook.add_worksheet();

    // Set the column weight
    worksheet.set_column_width(0, 8)?;
    worksheet.set_column_width(1, 25)?;
    worksheet.set_column_width(2, 15)?;
    worksheet.set_column_width(3, 25)?;

    let _ = worksheet.merge_range(
        0,
        0,
        0,
        3,
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
    ["#", "Продукт", "Ед.измерения", "Кол-во"].map(|title| {
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
        let _ = worksheet.write_with_format(i, 2, item.measure.clone(), &right_fmt);
        let _ = worksheet.write_with_format(
            i,
            3,
            item.cnt,
            &Format::new().set_border(FormatBorder::Thin),
        );

        i += 1;
    });

    // Итоги
    let _ = worksheet.write_with_format(i, 3, Formula::new(format!("=SUM(D3:D{})", i)), &bold_fmt);

    // Merge cells
    let _ = worksheet.merge_range(
        i,
        0,
        i,
        2,
        "Всего произведено товаров",
        &Format::new()
            .set_bold()
            .set_align(FormatAlign::Center)
            .set_border(FormatBorder::Thin),
    );

    let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
    let _ = wookbook.save(tmpfile.path());

    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    Ok(tmpfile)
}
