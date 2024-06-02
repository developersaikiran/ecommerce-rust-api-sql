use actix_web::{web, HttpResponse};

use super::{user, admin};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v1")
        .configure(user::auth::config)
        // .configure(user::notes::config)
    );
}