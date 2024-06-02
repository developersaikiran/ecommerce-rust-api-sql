use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestBody_RTypes {
    pub email: String,
    pub password: String,
    pub device_token: String,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FindUser_RType {
    pub id: i32,
    pub email: String,
    pub password: String,
    // pub device_token: String,
}