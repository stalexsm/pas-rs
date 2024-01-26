use axum::{
    extract::{Request, State},
    http::{self, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, patch, post},
    Router,
};
use backend::{
    endpoints::{
        init::{authorization, logout},
        produced_goods::{
            analitic::{get_analitics, upload_report_in_excel},
            produced_good::{
                add_adj_produced_goods, create_produced_good, delete_produced_good,
                detail_produced_good, edit_produced_good, get_produced_goods,
            },
        },
        rbs::{
            measure::{create_measure, delete_measure, detail_measure, edit_measure, get_measures},
            product::{create_product, delete_product, detail_product, edit_product, get_products},
        },
        users::{
            organization::{
                create_organization, detail_organization, edit_organization, get_organizations,
            },
            user::{create_user, current_user, detail_user, edit_passwd, edit_user, get_users},
        },
    },
    CurrentUser,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};
use uuid::Uuid;

use std::time::Duration;

#[tokio::main]
async fn main() {
    // Run Application

    // Ведение журнала
    tracing_subscriber::fmt().init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@127.0.0.1:54320/postgres".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .expect("can't connect to database");

    let api = Router::new()
        // Check Auth
        .route("/logout", post(logout))
        .route(
            "/organizations",
            get(get_organizations).post(create_organization),
        )
        .route(
            "/organizations/:id",
            get(detail_organization).patch(edit_organization),
        )
        .route("/current", get(current_user))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(detail_user).patch(edit_user))
        .route("/users/:id/passwd", patch(edit_passwd))
        .route("/measure-units", get(get_measures).post(create_measure))
        .route(
            "/measure-units/:id",
            get(detail_measure)
                .patch(edit_measure)
                .delete(delete_measure),
        )
        .route("/products", get(get_products).post(create_product))
        .route(
            "/products/:id",
            get(detail_product)
                .patch(edit_product)
                .delete(delete_product),
        )
        .route(
            "/produced-goods",
            get(get_produced_goods).post(create_produced_good),
        )
        .route(
            "/produced-goods/:id",
            get(detail_produced_good)
                .patch(edit_produced_good)
                .delete(delete_produced_good),
        )
        .route("/produced-goods/:id/adj", post(add_adj_produced_goods))
        .route("/analitics", get(get_analitics))
        .route("/upload-report", post(upload_report_in_excel))
        .route_layer(middleware::from_fn_with_state(pool.clone(), authenticate))
        // Not Check Auth
        .route("/auth", post(authorization));

    // build our application with some routes
    let app = Router::new()
        .route("/", get(welcome))
        .nest("/api", api)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .layer(CompressionLayer::new())
        .with_state(pool);

    // run it with hyper
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn welcome() -> &'static str {
    "Hello, Axum! Go Development PAS!"
}

async fn authenticate(
    State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Middleware Autheticate

    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header.replace("Bearer ", "")
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(State(pool), &auth_header).await {
        // вставьте текущего пользователя в расширение запроса, чтобы обработчик мог
        // извлечь его
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(
    State(pool): State<PgPool>,
    auth_token: &str,
) -> Option<CurrentUser> {
    // проверка пользователя по токену

    if let Ok(auth_token) = Uuid::parse_str(auth_token) {
        if let Ok(user) = sqlx::query_as!(
            CurrentUser,
            "select
            users.id,
            users.organization_id,
            users.role,
            users.email,
            users.fio,
            users.blocked,
            sessions.id as token
        from users
        inner join sessions on sessions.user_id = users.id
        where sessions.id = $1",
            auth_token
        )
        // .bind(auth_token)
        .fetch_optional(&pool)
        .await
        {
            user
        } else {
            None
        }
    } else {
        None
    }
}
