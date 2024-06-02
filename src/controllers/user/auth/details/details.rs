use crate::{
    database::{ AppState },
    response::{server_error_response, success_response, bad_request_response},
    middleware::rType::Claims
};
use actix_web::{
    web, HttpRequest, HttpResponse
};
use serde::{Deserialize, Serialize};
use super::rType::{
    RequestBody_RTypes,
    FindUser_RType,
};
use serde_json::json;


pub async fn user_details( request: RequestBody_RTypes, db_config: web::Data<AppState>, token_payload: Claims ) -> Result<HttpResponse, HttpResponse> {
    let pool = db_config.db.lock().unwrap();
    let email = token_payload.email;

    // let body_bytes = opts.body().await.unwrap().to_vec(); // Convert to bytes
    // let body_str = String::from_utf8(body_bytes).unwrap();

    let users_result = sqlx::query_as!(
        FindUser_RType,
        r#"
        SELECT id, email FROM users 
        WHERE email = $1"#, email
    )
    .fetch_one(&*pool)
    .await;

    match users_result {
        Ok(users) => {
            let response_data = json!({
                "id": users.id,
                "email": users.email,
            });
            Ok(HttpResponse::Ok().json(success_response(response_data, "success")))
        }
        Err(err) => {
            eprintln!("Error fetching users: {:?}", err);
            // Err(HttpResponse::InternalServerError().finish())

            Err(HttpResponse::BadRequest().json(bad_request_response({}, "User is not exists with this email.", err.to_string())))
        }
    }
}
