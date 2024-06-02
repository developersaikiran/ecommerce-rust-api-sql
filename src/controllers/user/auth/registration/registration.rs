use crate::{
    controllers::user::auth::registration::rType::FindRole_RType,
    database::AppState,
    response::{bad_request_response, server_error_response, success_response},
};

use actix_web::{web, HttpResponse};

use super::rType::{CreateUser_RType, FindUser_RType, RequestBody_RTypes};

use bcrypt;

pub async fn users_registration(
    opts: web::Json<RequestBody_RTypes>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, HttpResponse> {
    let pool = data.db.lock().unwrap();
    let name = opts.name.clone();
    let email = opts.email.clone();
    let password = opts.password.clone();
    let device_token = opts.device_token.clone();

    let users_result = sqlx::query_as!(
        FindUser_RType,
        r#"
        SELECT email FROM users 
        WHERE email = $1"#,
        email
    )
    .fetch_one(&*pool)
    .await;

    match users_result {
        Ok(_) => Ok(HttpResponse::BadRequest().json(bad_request_response(
            {},
            "Email is already exists",
            {},
        ))),
        Err(err) => {
            // let hashed_password = hash_password(password);
            let hashed_password = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
                Ok(res) => res,
                Err(_) => return Err(HttpResponse::InternalServerError().finish()),
            };

            println!("hashed_password {}", hashed_password);

            let get_role = sqlx::query_as!(
                FindRole_RType,
                r#"SELECT id FROM roles WHERE role_name = 'user' "#
            )
            .fetch_one(&*pool)
            .await;
        
            // Create user
            let create_user = sqlx::query!(
                r#"
                    INSERT INTO users (name, email, password, device_token) VALUES ($1, $2, $3, $4) RETURNING id, name, email
                "#,
                name,
                email,
                hashed_password,
                device_token
            )
            .fetch_one(&*pool)
            .await;

            match create_user {
                Ok(user) => {
                    let create_user_role = sqlx::query!(
                        r#"
                            INSERT INTO user_roles (user_id, role_id) VALUES ($1, $2)
                        "#,
                        user.id,
                        get_role.unwrap().id,
                    )
                    .execute(&*pool)
                    .await;

                    match create_user_role {
                        Ok(_) => {
                            let created_user = CreateUser_RType {
                                id: user.id,
                                name: user.name,
                                email: user.email,
                            };

                            Ok(HttpResponse::Ok().json(success_response(created_user, "success")))
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
