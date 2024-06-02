use crate::{
    database::AppState,
    middleware::rType::Claims,
    response::{bad_request_response, server_error_response, success_response},
    services::imageUpload::save_image_base64,
};

use actix_web::{web, HttpResponse};
use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use serde_json::json;
use sqlx::query;

use super::rType::RequestBody_RTypes;

#[derive(Debug, Clone)]
enum SqlParam {
    Str(String),
    BigDecimal(BigDecimal),
    I32(i32),
}

pub async fn update_product(
    product_id: i32,
    request: web::Json<RequestBody_RTypes>,
    db_config: web::Data<AppState>,
    token_payload: Claims,
) -> Result<HttpResponse, HttpResponse> {
    let pool = db_config.db.lock().unwrap();

    let store_id = token_payload.user_id;

    // Check if the product already exists
    let product_check = sqlx::query!(
        r#"
        SELECT name FROM products
        WHERE id = $1 and store_id = $2
        "#,
        product_id,
        store_id
    )
    .fetch_optional(&*pool)
    .await;

    match product_check {
        Ok(Some(_)) => {
            // let name = &request.name;
            // let price = &request.price;
            // let discount = &request.discount;
            // let description = &request.description;
            // let ingredients = &request.ingredients;
            // let how_to_use = &request.how_to_use;
            // let quantity = &request.quantity;
            // let product_image = &request.product_image;

            // Create an initial SQL query and parameters
            let mut sql_query = String::from("UPDATE products SET ");
            let mut params: Vec<SqlParam> = Vec::new();
            let mut param_index = 1;

            if let Some(name) = &request.name {
                sql_query.push_str(&format!("name = ${}, ", param_index));
                // params.push(name as &dyn sqlx::Encode<'_, _>);
                params.push(SqlParam::Str(name.clone()));
                param_index += 1;
            }

            if let Some(price) = &request.price {
                sql_query.push_str(&format!("price = ${}, ", param_index));
                // params.push(price as &dyn sqlx::Encode<'_, _>);
                params.push(SqlParam::BigDecimal(price.clone()));
                param_index += 1;
            }

            if let Some(discount) = &request.discount {
                sql_query.push_str(&format!("discount = ${}, ", param_index));
                // params.push(discount as &dyn sqlx::Encode<'_, _>);
                params.push(SqlParam::BigDecimal(discount.clone()));
                param_index += 1;
            }

            if let Some(description) = &request.description {
                sql_query.push_str(&format!("description = ${}, ", param_index));
                // params.push(description as &dyn sqlx::Encode<'_, _>);
                params.push(SqlParam::Str(description.clone()));
                param_index += 1;
            }

            if let Some(ingredients) = &request.ingredients {
                sql_query.push_str(&format!("ingredients = ${}, ", param_index));
                // params.push(ingredients as &dyn sqlx::Encode<'_, _>);
                params.push(SqlParam::Str(ingredients.clone()));
                param_index += 1;
            }

            if let Some(how_to_use) = &request.how_to_use {
                sql_query.push_str(&format!("how_to_use = ${}, ", param_index));
                // params.push(how_to_use as &dyn sqlx::Encode<'_, _>);
                params.push(SqlParam::Str(how_to_use.clone()));
                param_index += 1;
            }

            if let Some(quantity) = &request.quantity {
                sql_query.push_str(&format!("quantity = ${}, ", param_index));
                // params.push(quantity as &dyn sqlx::Encode<'_, _>);
                params.push(SqlParam::I32(quantity.clone()));
                param_index += 1;
            }

            if let Some(product_image) = &request.product_image {
                // image upload
                let uploadedImage = save_image_base64(product_image.clone()).await?;

                sql_query.push_str(&format!("image = ${}, ", param_index));
                // params.push(product_image as &dyn sqlx::Encode<'_, _>);
                params.push(SqlParam::Str(uploadedImage));
                param_index += 1;
            }

            // Remove trailing comma and add WHERE clause
            sql_query.pop();
            sql_query.pop();
            sql_query.push_str(&format!(" WHERE id = ${} AND store_id = ${}", param_index, param_index+1));

            println!("sql_query: {}", sql_query);
            
            let mut query = query(&sql_query);
            
            for param in params.iter() {
                query = match param {
                    SqlParam::Str(value) => query.bind(value),
                    SqlParam::BigDecimal(value) => query.bind(value),
                    SqlParam::I32(value) => query.bind(*value),
                };
            }
            
            query = query.bind(&product_id);
            query = query.bind(&store_id);
            
            println!("{}", sql_query);

            match query.execute(&*pool).await {
                Ok(_) => Ok(HttpResponse::Ok().json(success_response({}, "Product updated successfully"))),
                Err(err) => {
                    eprintln!("Error updating product: {:?}", err);
                    // Err(HttpResponse::InternalServerError().finish())
                    Err(HttpResponse::InternalServerError().json(server_error_response({}, "Error creating product", {})))
                }
            }
        }
        Ok(None) => {
            // If the product is not exists, return a bad request response
            Ok(HttpResponse::BadRequest().json(bad_request_response({}, "Product is not exists", {}, )))
        }
        Err(err) => {
            eprintln!("Error checking product existence: {:?}", err);
            Err(
                HttpResponse::InternalServerError().json(server_error_response( {}, "Error checking product existence", {}, )),
            )
        }
    }
}
