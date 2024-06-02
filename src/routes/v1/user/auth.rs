use crate::{
    database::AppState,
    response::{bad_request_response, server_error_response, success_response},
    services::response::{forbidden_response, invalid_token_response},
};

use actix_web::{
    delete,
    dev::{forward_ready, ServiceRequest, ServiceResponse, Transform},
    get, middleware, patch, post, web, Error, HttpMessage, HttpRequest, HttpResponse, Responder,
};

use crate::{
    controllers::user::{
        auth::{details, login, registration}
    },
    middleware::authentication::verify_user,
};

use serde::{Deserialize, Serialize};
use std::env;

use dotenv::dotenv;

use jsonwebtoken::{decode, errors::Error as JwtError, DecodingKey, Validation};

// async fn verify_user(req: &HttpRequest) -> Result<details::rType::Claims, HttpResponse> {

//     dotenv().ok();
//     let jwt_secret_key = env::var("AUTH_SECRET_KEY").unwrap();

//     if let Some(auth_header) = req.headers().get("authorization") {
//         if let Ok(auth) = auth_header.to_str() {
//             if auth.starts_with("Bearer ") {
//                 let token = auth.replace("Bearer ", "");
//                 match decode::<details::rType::Claims>(
//                     &token,
//                     &DecodingKey::from_secret(jwt_secret_key.as_bytes()),
//                     &Validation::default(),
//                 ) {
//                     Ok(token_data) => {
//                         // return Ok(HttpResponse::Ok().json(token_data.claims));
//                         return Ok(token_data.claims);
//                     }
//                     Err(err) => {
//                         eprintln!("Token decode error: {:?}", err);
//                         return Err(HttpResponse::BadRequest().json(invalid_token_response({}, "Invalid token", {})))
//                     }
//                 }
//             } else {
//                 return Err(HttpResponse::BadRequest().json(forbidden_response({}, "Token mallformeted", {})))
//             }
//         }
//     }
//     Err(HttpResponse::BadRequest().json(invalid_token_response({}, "No token provided", {})))
// }

#[get("/details")]
async fn users_details_handler(
    req: HttpRequest,
    request: web::Json<details::rType::RequestBody_RTypes>,
    db_config: web::Data<AppState>,
) -> impl Responder {
    match verify_user(&req).await {
        Ok(token_data) => {
            match details::details::user_details(request.into_inner(), db_config, token_data).await
            {
                Ok(users) => users,
                Err(response) => response,
            }
        }
        Err(error) => error,
    }
}

#[post("/login")]
pub async fn users_login_handler(
    opts: web::Json<login::rType::RequestBody_RTypes>,
    data: web::Data<AppState>,
) -> impl Responder {
    match login::login::users_login(opts, data).await {
        Ok(users) => users,
        Err(response) => response,
    }
}

#[post("/registration")]
pub async fn users_registration_handler(
    opts: web::Json<registration::rType::RequestBody_RTypes>,
    data: web::Data<AppState>,
) -> impl Responder {
    match registration::registration::users_registration(opts, data).await {
        Ok(users) => users,
        Err(response) => response,
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(users_login_handler)
        .service(users_registration_handler)
        .service(users_details_handler);

    conf.service(scope);
}
