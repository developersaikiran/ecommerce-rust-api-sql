use crate::{
    database::AppState, middleware::rType::Claims, response::{bad_request_response, server_error_response, success_response}, services::imageUpload::save_image_base64
};

use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use serde_json::json;

use super::rType::{RequestBody_RTypes};

fn to_utc_datetime(datetime_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let naive_datetime = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%dT%H:%M:%SZ")?;
    Ok(DateTime::<Utc>::from_utc(naive_datetime, Utc))
}

pub async fn add_product(
    request: web::Json<RequestBody_RTypes>,
    db_config: web::Data<AppState>,
    token_payload: Claims,
) -> Result<HttpResponse, HttpResponse> {
    let pool = db_config.db.lock().unwrap();

    let name = &request.name;
    let price = &request.price;
    let discount = &request.discount;
    let description = &request.description;
    let ingredients = &request.ingredients;
    let how_to_use = &request.how_to_use;
    let quantity = &request.quantity;
    let product_image = &request.product_image;

    let store_id = token_payload.user_id;

    // Check if the product already exists
    let product_check = sqlx::query!(
        r#"
        SELECT name FROM products
        WHERE name = $1
        "#,
        name
    )
    .fetch_optional(&*pool)
    .await;


    match product_check {
        Ok(Some(_)) => {
            // If the product exists, return a bad request response
            Ok(HttpResponse::BadRequest().json(bad_request_response(
                {},
                "Product already exists",
                {},
            )))
        }
        Ok(None) => {

            // image upload
            let uploadedImage = save_image_base64(product_image.clone()).await?;
                   
            // Create product
            let create_product = sqlx::query!(
                r#"
                    INSERT INTO products 
                    (name, store_id, price, discount, image, description, ingredients, how_to_use, quantity) 
                    VALUES 
                    ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
                    RETURNING id, name
                "#,
                name,
                store_id,
                price,
                discount,
                uploadedImage,
                description,
                ingredients,
                how_to_use,
                quantity
            )
            .fetch_one(&*pool)
            .await;

            match create_product {
                Ok(product) => {
                    let created_product = json!({
                        "id": product.id,
                        "name": product.name,
                    });
                    Ok(HttpResponse::Ok().json(success_response(created_product, "success")))
                }
                Err(err) => {
                    eprintln!("Error creating product: {:?}", err);
                    // Err(HttpResponse::InternalServerError().finish());
                    Err(HttpResponse::InternalServerError().json(server_error_response({}, "Error creating product", {})))
                }
            }
        }
        Err(err) => {
            eprintln!("Error checking product existence: {:?}", err);
            // Err(HttpResponse::InternalServerError().finish())
            Err(HttpResponse::InternalServerError().json(server_error_response({}, "Error checking product existence", {})))
        }
    }
}
