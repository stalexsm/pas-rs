use serde::{Deserialize, Serialize};

pub mod component;
pub mod list;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Analitic {
    pub id: i64,
    pub name: String,
    pub measure: String,
    pub fio: String,
    pub cnt: i64,
}
