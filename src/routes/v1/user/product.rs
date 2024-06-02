use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    controllers::user::product::products_list, database::AppState, response::{bad_request_response, server_error_response, success_response}, services::response::{forbidden_response, invalid_token_response}
};

use actix_multipart::Multipart;
use actix_web::{
    delete,
    dev::{forward_ready, ServiceRequest, ServiceResponse, Transform},
    get,
    http::header,
    middleware, patch, post, put, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use base64::decode;
use chrono::Utc;
use futures::StreamExt;

use crate::{
    controllers::user::{
        auth::{details, login, registration},
        product::{
            add_product,
            update_product,
            get_product
        },
    },
    middleware::authentication::verify_user,
};

use serde::{Deserialize, Serialize};

//------------------------ update product info ---------------------------------------- 
#[post("/add-product")]
async fn create_product(
    req: HttpRequest,
    request: web::Json<add_product::rType::RequestBody_RTypes>,
    db_config: web::Data<AppState>,
) -> impl Responder {
    match verify_user(&req).await {
        Ok(token_data) => {
            match add_product::add_product::add_product(request, db_config, token_data).await
            {
                Ok(users) => users,
                Err(response) => response,
            }
        }
        Err(error) => error,
    }
}

//------------------------ update product info ---------------------------------------- 
#[put("/update-product/{product_id}")]
async fn update_product_info(
    req: HttpRequest,
    path: web::Path<update_product::rType::UpdateProductPathParams>,
    request: web::Json<update_product::rType::RequestBody_RTypes>,
    db_config: web::Data<AppState>,
) -> impl Responder {
    match verify_user(&req).await {
        Ok(token_data) => {
            let product_id = path.product_id;
            match update_product::update_product::update_product(product_id, request, db_config, token_data).await
            {
                Ok(users) => users,
                Err(response) => response,
            }
        }
        Err(error) => error,
    }
}


//------------------------ Get product ---------------------------------------- 
#[get("/get-product/{product_id}")]
async fn get_product_info(
    req: HttpRequest,
    path: web::Path<get_product::rType::GetProductParams>,
    db_config: web::Data<AppState>,
) -> impl Responder {
    match verify_user(&req).await {
        Ok(token_data) => {
            let product_id = path.product_id;

            match get_product::get_product::get_product(product_id, db_config, token_data).await
            {
                Ok(users) => users,
                Err(response) => response,
            }
        }
        Err(error) => error,
    }
}


//------------------------ Get products list ---------------------------------------- 
#[get("/products-list")]
async fn get_products_list(
    req: HttpRequest,
    query: web::Query<products_list::rType::GetProductsListQueryParams>,
    db_config: web::Data<AppState>,
) -> impl Responder {
    match verify_user(&req).await {
        Ok(token_data) => {
            match products_list::products_list::products_list(query, db_config, token_data).await
            {
                Ok(users) => users,
                Err(response) => response,
            }
        }
        Err(error) => error,
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/product")
        .service(create_product)
        .service(update_product_info)
        .service(get_product_info)
        .service(get_products_list);

    conf.service(scope);
}
