use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use serde_json::json;
use uuid::Uuid;

pub mod endpoints;
pub mod services;

// Error
pub struct AppError(StatusCode, anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.0,
            Json(json!({
                "detail": format!("{}", self.1)
            })),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    // При преобразовании ошибки по умолчанию присваиваем код серверной ошибки
    fn from(err: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Role {
    Admin,
    User,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        // Преобразрвание в String
        match self {
            Role::User => "User".to_owned(),
            Role::Admin => "Admin".to_owned(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: i64,
    pub role: String,
    pub email: String,
    pub fio: Option<String>,
    pub blocked: bool,
    pub token: Uuid,
}
