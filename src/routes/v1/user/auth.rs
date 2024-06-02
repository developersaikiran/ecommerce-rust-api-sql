use crate::{
    database::{ AppState },
    response::{success_response},
};
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

use crate::{
    controllers:: {
        user::{
            login,
            registration,
        }
    }
};

use jsonwebtoken::{decode, DecodingKey, Validation, errors::Error as JwtError};
// const AUTH_SECRET_KEY: &[String] = std::env::var("AUTH_SECRET_KEY").to_string();
const AUTH_SECRET_KEY: &[u8] = b"AUTH_SECRET_KEY";

// async fn verify_user(req: HttpRequest) -> Result<HttpRequest, HttpResponse> {
//     if let Some(auth_header) = req.headers().get("authorization") {
//         if let Ok(auth) = auth_header.to_str() {
//             if auth.starts_with("Bearer ") {
//                 let token = auth.replace("Bearer ", "");
//                 match decode::<Claims>(
//                     &token,
//                     &DecodingKey::from_secret(token_secret_key.as_ref()),
//                     &Validation::default(),
//                 ) {
//                     Ok
//                 }
//             }
//         }
//     }
//     Err(HttpResponse::Unauthorized().finish())
// }

#[post("/login")]
pub async fn users_login_handler( 
    opts: web::Json<login::rType::RequestBody_RTypes>, 
    data: web::Data<AppState>, 
    auth: web::Data<AppState> 
) -> impl Responder {
    match login::login::users_login(opts, data).await {
        Ok(users) => users,
        Err(response) => response,
    }
}

#[post("/registration")]
pub async fn users_registration_handler( opts: web::Json<registration::rType::RequestBody_RTypes>, data: web::Data<AppState> ) -> impl Responder {
    match registration::registration::users_registration(opts, data).await {
        Ok(users) => users,
        Err(response) => response,
    }
}




pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/auth")
        .service(users_login_handler)
        .service(users_registration_handler);

    conf.service(scope);
}
