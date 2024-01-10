use serde::{Deserialize, Serialize};
use yew::{Reducible, UseReducerHandle};
use yew_router::prelude::*;

pub mod components;

#[derive(Clone, Serialize, Deserialize, Debug)]
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

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct User {
    pub id: i64,
    pub role: String,
    pub email: String,
    pub passwd: Option<String>,
    pub fio: Option<String>,
    pub blocked: bool,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
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
