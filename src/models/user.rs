#![allow(dead_code)]
use rs_uuid::uuid16;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub inserted_at: Option<String>, // generated by database on insert
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            uuid: uuid16(),
            username,
            password,
            inserted_at: None,
        }
    }
}
