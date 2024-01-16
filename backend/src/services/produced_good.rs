use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, PgPool, Row};

use crate::{check_is_admin, services::Items, AppError, CurrentUser};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    product_id: i64,
    cnt: i64,
}

pub async fn create_produced_good(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Json(body): Json<RequestBody>,
) -> Result<i64, anyhow::Error> {
    // Бизнес логика создания продукта

    let row: (i64,) = sqlx::query_as(
        "insert
            into produced_goods (product_id, user_id, cnt) values
            ($1, $2, $3) returning id",
    )
    .bind(body.product_id)
    .bind(current_user.id)
    .bind(body.cnt)
    .fetch_one(&pool)
    .await?;

    Ok(row.0)
}

pub async fn edit_produced_good(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBody>,
) -> Result<i64, AppError> {
    // Бизнес логика редактирования продукта

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let _ = sqlx::query(
            "update produced_goods
            set product_id=$1, cnt=$2, updated_at=NOW()
            where id = $3",
        )
        .bind(body.product_id)
        .bind(body.cnt)
        .bind(id)
        .execute(&pool)
        .await?;

        Ok(id)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Q {
    #[serde(default = "page")]
    pub page: i64,
    #[serde(default = "per_page")]
    pub per_page: i64,
}

fn per_page() -> i64 {
    15
}

fn page() -> i64 {
    1
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Select {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemProduct {
    pub id: i64,
    pub name: String,

    measure_unit: Select,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct USelect {
    pub id: i64,
    pub fio: Option<String>,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub cnt: i64,
    pub adj: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,

    product: ItemProduct,
    user: USelect,
}

pub async fn get_produced_goods(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<Q>,
) -> Result<Items<Item>, anyhow::Error> {
    // Бизнес логика получения списка продуктв

    let mut where_additional = String::new();
    if !check_is_admin(current_user.role) {
        let current_date = chrono::Utc::now().date_naive();
        where_additional.push_str(&format!(
            "and produced_goods.user_id = {0} and produced_goods.created_at between '{1} 00:00:00' and '{1} 23:59:59'",
            current_user.id, current_date
        ));
    }

    let sql = format!(
        r#"select
    produced_goods.id,
    produced_goods.cnt,
    produced_goods.created_at,
    products.id as product_id,
    products.name as product_name,
    measure_units.id as measure_unit_id,
    measure_units.name as measure_unit_name,
    users.id as user_id,
    users.fio as users_fio,
    users.email as users_email,
    sum(COALESCE(produced_good_adjustments.cnt::integer, 0)) as adj
from produced_goods
inner join users on users.id = produced_goods.user_id
inner join products on products.id = produced_goods.product_id
inner join measure_units on measure_units.id = products.measure_unit_id
left join produced_good_adjustments on produced_good_adjustments.produced_good_id = produced_goods.id
where true {}
group by produced_goods.id,
  produced_goods.cnt,
  produced_goods.created_at,
  products.id,
  products.name,
  measure_units.id,
  measure_units.name,
  users.id,
  users.fio,
  users.email
order by produced_goods.id desc
offset $1 limit $2;"#,
        where_additional
    );

    let rows = sqlx::query(&sql)
        .bind((q.page - 1) * q.per_page)
        .bind(q.per_page)
        .map(|row: PgRow| Item {
            id: row.get(0),
            cnt: row.get(1),
            adj: row.get(10),
            created_at: row.get(2),
            product: ItemProduct {
                id: row.get(3),
                name: row.get(4),
                measure_unit: Select {
                    id: row.get(5),
                    name: row.get(6),
                },
            },
            user: USelect {
                id: row.get(7),
                fio: row.get(8),
                email: row.get(9),
            },
        })
        .fetch_all(&pool)
        .await?;

    // Подсчет данных для пагинации
    let sql = format!(
        "select count(id) from produced_goods where true {};",
        where_additional
    );
    let cnt: i64 = sqlx::query_scalar(&sql).fetch_one(&pool).await.unwrap_or(0);
    let cnt = (cnt as f64 / q.per_page as f64).ceil() as i64;

    Ok(Items { items: rows, cnt })
}

pub async fn detail_produced_good(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<Item, AppError> {
    // Бизнес логика получения продуктв

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let row = sqlx::query(
            r#"select
            produced_goods.id,
            produced_goods.cnt,
            produced_goods.created_at,
            products.id as product_id,
            products.name as product_name,
            measure_units.id as measure_unit_id,
            measure_units.name as measure_unit_name,
            users.id as user_id,
            users.fio as users_fio,
            users.email as users_email,
            sum(produced_good_adjustments.cnt::integer) as adj
        from produced_goods
        inner join users on users.id = produced_goods.user_id
        inner join products on products.id = produced_goods.product_id
        inner join measure_units on measure_units.id = products.measure_uint_id
        inner join produced_good_adjustments on produced_good_adjustments.produced_good_id = produced_goods.id
        group by produced_goods.id,
          produced_goods.cnt,
          produced_goods.created_at,
          products.id,
          products.name,
          measure_units.id,
          measure_units.name,
          users.id,
          users.fio,
          users.email
        where products.id = $1;"#,
        )
        .bind(id)
        .fetch_optional(&pool)
        .await?;

        match row {
            // Собираем в нужный вид
            Some(row) => Ok(Item {
                id: row.get(0),
                cnt: row.get(1),
                adj: row.get(10),
                created_at: row.get(2),
                product: ItemProduct {
                    id: row.get(3),
                    name: row.get(4),
                    measure_unit: Select {
                        id: row.get(5),
                        name: row.get(6),
                    },
                },
                user: USelect {
                    id: row.get(7),
                    fio: row.get(8),
                    email: row.get(9),
                },
            }),
            None => Err(AppError(
                StatusCode::NOT_FOUND,
                anyhow::anyhow!("Такой записи не существует"),
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBodyAdj {
    cnt: i64,
}

pub async fn add_adj_produced_goods(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
    Json(body): Json<RequestBodyAdj>,
) -> Result<i64, anyhow::Error> {
    // Бизнес логика создания продукта

    let row: (i64,) = sqlx::query_as(
        "insert
            into produced_good_adjustments (user_id, produced_good_id, cnt) values
            ($1, $2, $3) returning id",
    )
    .bind(current_user.id)
    .bind(id)
    .bind(body.cnt)
    .fetch_one(&pool)
    .await?;

    Ok(row.0)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Analitics {
    pub id: i64,
    pub name: String,
    pub measure: String,
    pub cnt: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QA {
    pub date_one: chrono::NaiveDate,
    pub date_two: chrono::NaiveDate,
}

pub async fn get_analitics(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Query(q): Query<QA>,
) -> Result<Vec<Analitics>, AppError> {
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
        .map(|row: PgRow| Analitics {
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
