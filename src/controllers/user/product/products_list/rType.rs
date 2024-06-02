use chrono;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use sqlx::prelude::FromRow;

#[allow(non_snake_case)]
#[derive(Serialize, FromRow, Debug)]
pub struct Product_RTypes {
    pub id: Option<i32>,
    pub store_id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<BigDecimal>,
    pub discount: Option<BigDecimal>,
    pub description: Option<String>,
    pub ingredients: Option<String>,
    pub how_to_use: Option<String>,
    pub quantity: Option<i32>,
    pub image: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}


#[derive(Deserialize)]
pub struct GetProductsListQueryParams {
    pub product_ids: Option<Vec<String>>,
    pub search: Option<String>,
    pub min_price: Option<BigDecimal>,
    pub max_price: Option<BigDecimal>,
    pub newest: Option<bool>,
}
