use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestBody_RTypes {
    pub name: String,
    pub email: String,
    pub password: String,
    pub device_token: String,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FindUser_RType {
    pub email: String,
    // pub device_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FindRole_RType {
    pub id: i32,
    // pub email: String
    // pub device_token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUser_RType {
    pub id: i32,
    pub name: String,
    pub email: String,
    // pub device_token: String,
}