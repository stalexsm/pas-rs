use serde::{Deserialize, Serialize};

use crate::Select;

pub mod component;
pub mod list;
pub mod modal;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MeasureUnit {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,

    pub organization: Select,
}
