use crate::{
    database::AppState,
    response::{bad_request_response, server_error_response, success_response}, 
    services::mail::send_registration_email,
};

use actix_web::{
    web,
    HttpResponse,
};

use super::rType::{
    RequestBody_RTypes,
    FindUser_RType,
};

use serde_json::json;
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};

use std::{env, u32};
use dotenv::dotenv;

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};

#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: i32,
    email: String,
    exp: usize,
}
use bcrypt;

pub fn create_token(user_id: &i32, email: &String) -> String {
    dotenv().ok();
    // const AUTH_SECRET_KEY: &[u8] = env::var("AUTH_SECRET_KEY").unwrap().as_bytes();
    let jwt_secret_key = env::var("AUTH_SECRET_KEY").unwrap();
    // let auth_secret_key = env::var("AUTH_SECRET_KEY").unwrap().as_bytes().to_owned();
    // println!("{}", jwt_secret_key.as_bytes());
    // return "123".to_string();
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims { 
        user_id: user_id.to_owned(), 
        email: email.to_string(), 
        exp: expiration as usize };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret_key.as_bytes())).unwrap()
}

pub async fn users_login( opts: web::Json<RequestBody_RTypes>, data: web::Data<AppState> ) -> Result<HttpResponse, HttpResponse> {
    let pool = data.db.lock().unwrap();
    let email = opts.email.clone();
    let password = opts.password.clone();
    println!("email: {}", email);

    let users_result = sqlx::query_as!(
        FindUser_RType,
        r#"
        SELECT id, email, password FROM users 
        WHERE email = $1"#, email
    )
    .fetch_one(&*pool)
    .await;

    match users_result {
        Ok(users) => {
            if bcrypt::verify(password.as_str(), users.password.as_str()).unwrap_or(false) {
                // Generate JWT token
                let token = create_token(&users.id, &email);

                // let email = "exampleuser2024@yopmail.com"; // Replace with the user's email
                // let subject = "Welcome to Our Service!";
                // let body = "Thank you for registering with us.";
                
                // if let Err(e) = send_registration_email(email, subject, body).await {
                //     eprintln!("Failed to send registration email: {:?}", e);
                // }

                let response_data = json!({
                    "user": {
                        "id": users.id,
                        "email": users.email,
                    },
                    "token": token,
                });

                Ok(HttpResponse::Ok().json(success_response(response_data, "success")))
            } else{
                Ok(HttpResponse::BadRequest().json(bad_request_response({}, "Invalid password", {})))
            }

            // Ok(HttpResponse::Ok().json(success_response(users, "success")))
        }
        Err(err) => {
            eprintln!("Error fetching users: {:?}", err);
            // Err(HttpResponse::InternalServerError().finish())

            Err(HttpResponse::BadRequest().json(bad_request_response({}, "User is not exists with this email.", err.to_string())))
        }
    }
}
