use std::env;

use actix_web::{HttpRequest, HttpResponse};
use dotenv::dotenv;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    controllers::user::auth::details, 
    services::response::{
        forbidden_response, invalid_token_response
    },
    middleware::rType::Claims
};



pub async fn verify_user(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    
    dotenv().ok();
    let jwt_secret_key = env::var("AUTH_SECRET_KEY").unwrap();

    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth) = auth_header.to_str() {
            if auth.starts_with("Bearer ") {
                let token = auth.replace("Bearer ", "");
                match decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(jwt_secret_key.as_bytes()),
                    &Validation::default(),
                ) {
                    Ok(token_data) => {
                        // return Ok(HttpResponse::Ok().json(token_data.claims));
                        return Ok(token_data.claims);
                    }
                    Err(err) => {
                        eprintln!("Token decode error: {:?}", err);
                        return Err(HttpResponse::BadRequest().json(invalid_token_response({}, "Invalid token", {})))
                    }
                }
            } else {
                return Err(HttpResponse::BadRequest().json(forbidden_response({}, "Token mallformeted", {})))
            }
        }
    }
    Err(HttpResponse::BadRequest().json(invalid_token_response({}, "No token provided", {})))
}
