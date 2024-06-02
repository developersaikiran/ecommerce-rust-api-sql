use actix_web::{web, HttpResponse};

use super::v1;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(v1::index::config));
}
