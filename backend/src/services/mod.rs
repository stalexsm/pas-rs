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
