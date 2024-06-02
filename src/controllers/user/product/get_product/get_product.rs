use crate::{
    controllers::user::product::products_list::rType::Product_RTypes, database::AppState, middleware::rType::Claims, response::{bad_request_response, server_error_response, success_response}, services::imageUpload::save_image_base64
};

use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use futures::SinkExt;
use serde_json::json;
use sqlx::query;

use super::rType::{GetProductParams};

#[derive(Debug, Clone)]
enum SqlParam {
    Str(String),
    BigDecimal(BigDecimal),
    Boolean(bool),
}

pub async fn get_product(
    product_id: i32,
    db_config: web::Data<AppState>,
    token_payload: Claims,
) -> Result<HttpResponse, HttpResponse> {

    let pool = db_config.db.lock().unwrap();
    let store_id = token_payload.user_id;

    // Create an initial SQL query and parameters
    let mut sql_query = String::from("SELECT *, CAST(created_at AS VARCHAR(50)), CAST(updated_at AS VARCHAR(50)) FROM products WHERE id=$1");

    let mut query = sqlx::query_as::<_, Product_RTypes>(&sql_query);
    query = query.bind(product_id);
    
    println!("{}", sql_query);

    match query.fetch_one(&*pool).await {
        Ok(products) => {
            Ok(HttpResponse::Ok().json(success_response(products, "Product updated successfully")))
        },
        Err(err) => {
            eprintln!("Error updating product: {:?}", err);
            Err(HttpResponse::InternalServerError().json(server_error_response({}, "Error creating product", {})))
        }
    }
}
