use serde::{Deserialize, Serialize};

pub mod init;
pub mod produced_goods;
pub mod rbs;
pub mod users;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Items<T> {
    pub cnt: i64,
    pub items: Vec<T>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Select {
    pub id: i64,
    pub name: String,
}

impl From<serde_json::Value> for Select {
    fn from(value: serde_json::Value) -> Self {
        let s = value.to_string();

        match serde_json::from_str::<Select>(&s).ok() {
            Some(o) => o,
            _ => Default::default(),
        }
    }
}
