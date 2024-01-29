use core::fmt;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};
use yew::{Reducible, UseReducerHandle};
use yew_router::prelude::*;

pub mod components;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum Role {
    Developer,
    Admin,
    Director,

    #[default]
    User,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

pub fn check_is_admin(role: Role) -> bool {
    // Вспомогательная функция для проверки админских ролей

    matches!(role, Role::Developer | Role::Admin)
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/auth")]
    Auth,
    #[at("/")]
    Home,
    #[at("/users")]
    User,
    #[at("/products")]
    Product,
    #[at("/measure-units")]
    MeasureUnit,
    #[at("/analitics")]
    Analitic,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ResponseItems<T> {
    cnt: i64,
    items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMsg {
    detail: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseId {
    id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Select {
    pub id: i64,
    pub name: String,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct User {
    pub id: i64,
    pub role: Role,
    pub email: String,
    pub passwd: Option<String>,
    pub fio: String,
    pub blocked: bool,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,

    pub organization: Option<Select>,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct AppStateContext(pub Option<User>);

impl Reducible for AppStateContext {
    type Action = Option<User>;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        AppStateContext(action).into()
    }
}

pub type AppContext = UseReducerHandle<AppStateContext>;

fn _empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}
