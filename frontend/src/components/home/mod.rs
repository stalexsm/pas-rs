use serde::{Deserialize, Serialize};

use crate::Select;

pub mod component;
pub mod list;
pub mod modal;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct USelect {
    pub id: i64,
    pub fio: String,
    pub email: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemProduct {
    pub id: i64,
    pub name: String,
    pub measure_unit: Select,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProducedGood {
    pub id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub cnt: i64,
    pub adj: i64,
    pub user: USelect,
    pub product: ItemProduct,
    pub organization: Select,
}
