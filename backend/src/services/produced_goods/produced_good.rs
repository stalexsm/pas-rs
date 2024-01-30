use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{check_is_admin, services::Items, AppError, CurrentUser, Role};

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
        "INSERT INTO
          produced_goods (product_id, user_id, cnt, organization_id)
        VALUES
          (
            $1,
            $2,
            $3,
            CASE
              WHEN $4::bigint IS NULL THEN (
                SELECT
                  p.organization_id
                FROM
                  products as p
                WHERE
                  p.id = $1
              )
              ELSE $4
            END
          )
        RETURNING
          id",
    )
    .bind(body.product_id)
    .bind(current_user.id)
    .bind(body.cnt)
    .bind(current_user.organization_id)
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
            "UPDATE
              produced_goods
            SET
              product_id = $1,
              cnt = $2,
              organization_id = CASE
                WHEN $4::bigint IS NULL THEN (
                  SELECT
                    p.organization_id
                  FROM
                    products as p
                  WHERE
                    p.id = $1
                )
                ELSE $4
              END,
              updated_at = NOW()
            WHERE
              id = $3",
        )
        .bind(body.product_id)
        .bind(body.cnt)
        .bind(id)
        .bind(current_user.organization_id)
        .execute(&pool)
        .await?;

        Ok(id)
    }
}

pub async fn delete_produced_good(
    State(pool): State<PgPool>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    // Бизнес логика удаления производства

    if !check_is_admin(current_user.role) {
        Err(AppError(
            StatusCode::FORBIDDEN,
            anyhow::anyhow!("У вас нет доступа для данного действия!"),
        ))
    } else {
        let _ = sqlx::query(
            "DELETE
        FROM produced_goods
        WHERE id = $1;",
        )
        .bind(id)
        .execute(&pool)
        .await?;

        Ok(())
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
    pub fio: String,
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

    let mut current_date: Option<NaiveDate> = None;
    let mut current_user_id: Option<i64> = None;
    if !check_is_admin(current_user.role) && current_user.role != Role::Director {
        current_date = Some(chrono::Utc::now().date_naive());
        current_user_id = Some(current_user.id);
    }

    let rows = sqlx::query!(
        "SELECT
    pg.id,
    pg.cnt,
    pg.created_at,
    p.id AS product_id,
    p.name AS product_name,
    mu.id AS measure_unit_id,
    mu.name AS measure_unit_name,
    u.id AS user_id,
    u.fio AS user_fio,
    u.email AS user_email,
    SUM(COALESCE(pga.cnt::INTEGER, 0)) AS adj
FROM produced_goods AS pg
INNER JOIN users AS u on u.id = pg.user_id
INNER JOIN products AS p  on p.id = pg.product_id
INNER JOIN measure_units AS mu on mu.id = p.measure_unit_id
LEFT JOIN produced_good_adjustments AS pga on pga.produced_good_id = pg.id
WHERE
    CASE
        WHEN $1::bigint IS NOT NULL THEN
            pg.user_id = $1 AND pg.created_at::date = $2
        WHEN $3::bigint IS NOT NULL AND $4 = 'Director' THEN
            pg.organization_id = $3
        ELSE TRUE
    END
GROUP BY pg.id,
  pg.cnt,
  pg.created_at,
  p.id,
  p.name,
  mu.id,
  mu.name,
  u.id,
  u.fio,
  u.email
order by pg.id desc
offset $5 limit $6;",
        current_user_id,
        current_date,
        current_user.organization_id,
        current_user.role.to_string(),
        (q.page - 1) * q.per_page,
        q.per_page
    )
    .map(|row| Item {
        id: row.id,
        cnt: row.cnt,
        adj: row.adj.map_or(0, |a| a),
        created_at: row.created_at,
        product: ItemProduct {
            id: row.product_id,
            name: row.product_name,
            measure_unit: Select {
                id: row.measure_unit_id,
                name: row.measure_unit_name,
            },
        },
        user: USelect {
            id: row.user_id,
            fio: row.user_fio,
            email: row.user_email,
        },
    })
    .fetch_all(&pool)
    .await?;

    let cnt: i64 = sqlx::query_scalar(
        "SELECT COUNT(id) FROM produced_goods WHERE CASE
        WHEN $1::bigint IS NOT NULL THEN
            pg.user_id = $1 AND pg.created_at::date = $2
        ELSE TRUE
    END",
    )
    .bind(current_user_id)
    .bind(current_date)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);
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
        let row = sqlx::query!(
            "SELECT
        pg.id,
        pg.cnt,
        pg.created_at,
        p.id AS product_id,
        p.name AS product_name,
        mu.id AS measure_unit_id,
        mu.name AS measure_unit_name,
        u.id AS user_id,
        u.fio AS user_fio,
        u.email AS user_email,
        SUM(COALESCE(pga.cnt::INTEGER, 0)) AS adj
    FROM produced_goods AS pg
    INNER JOIN users AS u on u.id = pg.user_id
    INNER JOIN products AS p  on p.id = pg.product_id
    INNER JOIN measure_units AS mu on mu.id = p.measure_unit_id
    LEFT JOIN produced_good_adjustments AS pga on pga.produced_good_id = pg.id
    WHERE p.id = $1
    GROUP BY pg.id,
      pg.cnt,
      pg.created_at,
      p.id,
      p.name,
      mu.id,
      mu.name,
      u.id,
      u.fio,
      u.email;",
            id
        )
        .fetch_optional(&pool)
        .await?;

        match row {
            // Собираем в нужный вид
            Some(row) => Ok(Item {
                id: row.id,
                cnt: row.cnt,
                adj: row.adj.map_or(0, |a| a),
                created_at: row.created_at,
                product: ItemProduct {
                    id: row.product_id,
                    name: row.product_name,
                    measure_unit: Select {
                        id: row.measure_unit_id,
                        name: row.measure_unit_name,
                    },
                },
                user: USelect {
                    id: row.user_id,
                    fio: row.user_fio,
                    email: row.user_email,
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
        "INSERT
            INTO produced_good_adjustments (user_id, produced_good_id, cnt) VALUES
            ($1, $2, $3) RETURNING id",
    )
    .bind(current_user.id)
    .bind(id)
    .bind(body.cnt)
    .fetch_one(&pool)
    .await?;

    Ok(row.0)
}
