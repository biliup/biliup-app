use crate::video::Studio;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::Map;

pub mod error;
pub mod video;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub account: Account,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user: User,
    pub streamers: HashMap<String, Studio>,
}
