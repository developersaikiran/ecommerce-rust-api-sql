use crate::{
    controllers::user::product::products_list::rType::Product_RTypes, database::AppState, middleware::rType::Claims, response::{bad_request_response, server_error_response, success_response}, services::imageUpload::save_image_base64
};

use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use futures::SinkExt;
use serde_json::json;
use sqlx::query;

use super::rType::{GetProductsListQueryParams};

#[derive(Debug, Clone)]
enum SqlParam {
    Str(String),
    BigDecimal(BigDecimal),
    Boolean(bool),
}

pub async fn products_list(
    queryParams: web::Query<GetProductsListQueryParams>,
    db_config: web::Data<AppState>,
    token_payload: Claims,
) -> Result<HttpResponse, HttpResponse> {

    let pool = db_config.db.lock().unwrap();
    let store_id = token_payload.user_id;

    // Create an initial SQL query and parameters
    let mut sql_query = String::from("SELECT *, CAST(created_at AS VARCHAR(50)), CAST(updated_at AS VARCHAR(50)) FROM products WHERE 1=1");
    let mut params: Vec<SqlParam> = Vec::new();
    let mut param_index = 1;

    if let Some(search) = &queryParams.search {
        sql_query.push_str(&format!(" AND name ILIKE ${} ", param_index));
        params.push(SqlParam::Str(format!("%{}%", search)));
        param_index += 1;
    }

    if let Some(min_price) = &queryParams.min_price {
        sql_query.push_str(&format!(" AND price >= ${} ", param_index));
        params.push(SqlParam::BigDecimal(min_price.clone()));
        param_index += 1;
    }

    if let Some(max_price) = &queryParams.max_price {
        sql_query.push_str(&format!(" AND price <= ${} ", param_index));
        params.push(SqlParam::BigDecimal(max_price.clone()));
        param_index += 1;
    }

    if queryParams.newest.unwrap_or(false) {
        sql_query.push_str(&format!("ORDER BY created_at DESC"));
    }

    println!("sql_query: {}", sql_query);

    // let mut query = query(&sql_query);
    let mut query = sqlx::query_as::<_, Product_RTypes>(&sql_query);

    for param in params.iter() {
        query = match param {
            SqlParam::Str(value) => query.bind(value),
            SqlParam::Boolean(value) => query.bind(value),
            SqlParam::BigDecimal(value) => query.bind(value),
        };
    }

    println!("{}", sql_query);

    match query.fetch_all(&*pool).await {
        Ok(products) => {
            // println!("{:?}", data);
            Ok(HttpResponse::Ok().json(success_response(products, "Product updated successfully")))
        },
        Err(err) => {
            eprintln!("Error updating product: {:?}", err);
            // Err(HttpResponse::InternalServerError().finish())
            Err(HttpResponse::InternalServerError().json(server_error_response({}, "Error creating product", {})))
        }
    }
}
