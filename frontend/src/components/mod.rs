use serde::{Deserialize, Serialize};

pub mod analitic;
pub mod auth;
pub mod elements;
pub mod header;
pub mod home;
pub mod not_found;
pub mod rbs;
pub mod user;

// Для пагинации
const PER_PAGE: i64 = 5;

pub trait SelectableItem {
    // Для возможности использовать объект для select

    fn id(&self) -> i64;
    fn name(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Select {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseError {
    pub detail: String,
}
