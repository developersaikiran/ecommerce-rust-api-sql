use crate::{
    database::{ AppState },
    response::{server_error_response, success_response, bad_request_response},
};

use actix_web::{
    web,
    HttpResponse,
};

use super::rType::{
    RequestBody_RTypes,
    FindUser_RType,
    CreateUser_RType,
};

use bcrypt;

pub async fn users_registration( opts: web::Json<RequestBody_RTypes>, data: web::Data<AppState> ) -> Result<HttpResponse, HttpResponse> {
    let pool = data.db.lock().unwrap();
    let name = opts.name.clone();
    let email = opts.email.clone();
    let password = opts.password.clone();
    let device_token = opts.device_token.clone();

    let users_result = sqlx::query_as!(
        FindUser_RType,
        r#"
        SELECT email FROM users 
        WHERE email = $1"#, email
    )
    .fetch_one(&*pool)
    .await;

    match users_result {
        Ok(_) => Ok(HttpResponse::BadRequest().json(
            bad_request_response({}, "Email is already exists", {}),
        )),
        Err(err) => {

            // let hashed_password = hash_password(password);
            let hashed_password = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
                Ok(res) => res,
                Err(_) => return Err(HttpResponse::InternalServerError().finish())
            };

            println!("hashed_password {}", hashed_password);

            let create_user = sqlx::query!(
                r#"
                    INSERT INTO users (name, email, password, device_token) VALUES ($1, $2, $3, $4)
                "#,
                name,
                email,
                hashed_password,
                device_token
            )
            .execute(&*pool)
            .await;


            match create_user {
                Ok(_) => {
                    // Fetch the user after successful insertion
                    let registered_user = sqlx::query_as!(
                        CreateUser_RType,
                        r#"
                            SELECT id, name, email FROM users WHERE email = $1
                        "#,
                        email
                    )
                    .fetch_one(&*pool)
                    .await;

                    match registered_user {
                        Ok(user) => {
                            Ok(HttpResponse::Ok().json(success_response(user, "success")))
                        },
                        Err(err) => {
                            eprintln!("Error fetching user: {:?}", err);
                            Err(HttpResponse::InternalServerError().finish())
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error creating user: {:?}", err);
                    Err(HttpResponse::InternalServerError().finish())
                }
            }
        }
    }
}
