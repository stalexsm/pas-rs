use serde::{Deserialize, Serialize};

pub mod init;
pub mod produced_good;
pub mod rbs;
pub mod user;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Items<T> {
    pub cnt: i64,
    pub items: Vec<T>,
}
