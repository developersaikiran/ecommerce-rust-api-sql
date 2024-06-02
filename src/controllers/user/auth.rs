use crate::{
    model::{AppState, QueryOptions, UpdateUserSchema, User},
    response::{server_error_response, success_response},
};
use actix_web::{
    web,
    Responder,
    HttpResponse,
};



pub async fn users_lists(
    opts: web::Query<QueryOptions>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, HttpResponse> {
    let pool = data.db.lock().unwrap();
    let limit = opts.limit.unwrap_or(10);
    let skip = opts.skip.unwrap_or(0);

    let users_result = sqlx::query_as!(
        User,
        r#"
        SELECT 
            users.id, 
            users.name, 
            users.email, 
            users.password, 
            users.createdAt, 
            users.updatedAt,
            JSON_OBJECT('name', users.name) AS user_roles
        FROM users 
        INNER JOIN user_roles ON user_roles.user_id = users.id
        LIMIT ? OFFSET ?"#,
        limit as i64,
        skip as i64
    )
    .fetch_one(&*pool)
    .await;

    match users_result {
        Ok(users) => {

            // return Ok(HttpResponse::InternalServerError().json(server_error_response({}, "User is already exits")));

            // if users.name == String::from("saikiran") {
            //     println!("This is my data: {:?}", users);
            //     return Ok(HttpResponse::Ok().json(server_error_response({}, "User is already exits")));
            // } 

            let json_response = match success_response(users, "success") {
                Ok(response) => response,
                Err(err) => {
                    eprintln!("Error creating success response: {}", err);
                    return Err(HttpResponse::InternalServerError().finish());
                }
            };

            Ok(HttpResponse::Ok().json(json_response))
        }
        Err(err) => {
            eprintln!("Error fetching users: {:?}", err);
            Err(HttpResponse::InternalServerError().finish())
        }
    }
}
