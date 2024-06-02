use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RequestBody_RTypes {
    pub name: Option<String>,
    pub price: Option<BigDecimal>,
    pub discount: Option<BigDecimal>,
    pub description: Option<String>,
    pub ingredients: Option<String>,
    pub how_to_use: Option<String>,
    pub quantity: Option<i32>,
    pub product_image: Option<String>
}


#[derive(Deserialize)]
pub struct UpdateProductPathParams {
    pub product_id: i32,
}

