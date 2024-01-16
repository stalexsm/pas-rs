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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "role")]
pub enum Role {
    Developer,
    Admin,
    Director,
    User,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        // Получение перечисления Ролей из ссылки на строку
        match value {
            "Developer" => Role::Developer,
            "Admin" => Role::Admin,
            "Director" => Role::Director,
            "User" => Role::Developer,
            _ => Role::User,
        }
    }
}

impl From<String> for Role {
    fn from(value: String) -> Self {
        // Получение перечисления Ролей из строки
        Role::from(value.as_str())
    }
}

pub fn check_is_admin(role: Role) -> bool {
    // Вспомогательная функция для проверки админских ролей

    matches!(role, Role::Developer | Role::Admin)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: i64,
    pub role: Role,
    pub email: String,
    pub fio: Option<String>,
    pub blocked: bool,
    pub token: Uuid,
}
