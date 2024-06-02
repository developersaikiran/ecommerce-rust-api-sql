use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestBody_RTypes {
    pub email: String,
}
pub struct RequestBody_RTypes_withToken {
    pub email: String,
    pub token_payload: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub user_id: i32,
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FindUser_RType {
    pub id: i32,
    pub email: String,
    // pub device_token: String,
}