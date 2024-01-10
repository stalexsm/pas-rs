use crate::components::Select;
use serde::{Deserialize, Serialize};

pub mod component;
pub mod list;
pub mod modal;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub measure_unit: Select,
}
