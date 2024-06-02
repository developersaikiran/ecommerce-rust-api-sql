use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};

mod config;
use crate::config::{ database };
use database::AppState;

mod services;
use crate::services::{response};
// use model::AppState;

pub mod controllers;
use crate::controllers::{user};

mod routes;
use crate::routes::{index};

use env_logger::Env;
use std::fs::{self, OpenOptions};
use dotenv::dotenv;



// ----------------------------- Setup Log file ----------------------------------
fn setup_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .write_style(env_logger::WriteStyle::Always)
        .init();
}

// ----------------------------- Create Log ----------------------------------
fn create_log_file() -> std::io::Result<std::fs::File> {
    // Create the "logs" directory if it doesn't exist
    if let Err(err) = fs::create_dir("src/logs") {
        if err.kind() != std::io::ErrorKind::AlreadyExists {
            eprintln!("Failed to create 'logs' directory: {}", err);
            std::process::exit(1);
        }
    }

    // Set up file for logging
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("src/logs/{}.log", chrono::Utc::now().format("%Y-%m-%d")))
        .expect("Failed to open log file");

    Ok(log_file)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = std::env::var("PORT_DEV").unwrap_or("8000".to_string());
    let host = std::env::var("HOST_DEV").unwrap_or("8000".to_string());

    // setup_logger();
    // let log_file = create_log_file().expect("Failed to create log file");

    let db = AppState::init().await.expect("Failed to initialize app state");
    let app_data = web::Data::new(db);

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:3000/")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .configure(index::config)
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind(format!("{host}:{port}"))?
    .run()
    .await   
}