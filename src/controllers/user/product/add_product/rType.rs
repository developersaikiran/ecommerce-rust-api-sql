use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestBody_RTypes {
    pub name: String,
    pub price: BigDecimal,
    pub discount: BigDecimal,
    pub description: String,
    pub ingredients: String,
    pub how_to_use: String,
    pub quantity: i32,
    pub product_image: String
}