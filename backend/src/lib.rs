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

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Role {
    Developer,
    Admin,
    Director,

    #[default]
    User,
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        // Получение перечисления Ролей из ссылки на строку
        match value {
            "Developer" => Role::Developer,
            "Admin" => Role::Admin,
            "Director" => Role::Director,
            "User" => Role::User,
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

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Developer => "Developer".to_string(),
            Role::Admin => "Admin".to_string(),
            Role::Director => "Director".to_string(),
            Role::User => "User".to_string(),
        }
    }
}

pub fn check_is_admin(role: Role) -> bool {
    // Вспомогательная функция для проверки админских ролей

    matches!(role, Role::Developer | Role::Admin)
}

pub fn check_access(role: Role) -> bool {
    // Вспомогательная функция для проверки доступа к функционалу

    check_is_admin(role) || role == Role::Director
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: i64,
    pub organization_id: Option<i64>,
    pub role: Role,
    pub email: String,
    pub fio: Option<String>,
    pub blocked: bool,
    pub token: Uuid,
}
