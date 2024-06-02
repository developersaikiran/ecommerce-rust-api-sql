use actix_multipart::Multipart;
use actix_web::error::ErrorBadRequest;
use actix_web::web::Json;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use base64::decode;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

use crate::services::response::bad_request_response;

// Define a struct for your error response
#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    data: serde_json::Value,
    message: String,
    err: serde_json::Value,
}

// Function to return a JSON response with a custom error message
fn error_response(data: serde_json::Value, message: &str, err: serde_json::Value) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        data,
        message: message.to_string(),
        err,
    })
}

pub async fn save_image_base64(image: String) -> Result<String, HttpResponse> {
    println!("Received request: {:?}", image);
    
    let is_valid_base64 = image.chars().all(|c| {
        matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '+' | '/' | '=')
    });

    if !is_valid_base64 {
        eprintln!("Invalid base64 string");
        return Err(HttpResponse::BadRequest().json(bad_request_response({}, "Invalid base64 string", {})));
    }

    // Decode the base64 string
    let decoded_data = decode(image);
    let decoded_data = match decoded_data {
        Ok(data) => data,
        Err(err) => {
            // Handle decoding error
            eprintln!("Failed to decode base64: {:?}", err);
            return Err(HttpResponse::BadRequest().json(bad_request_response({}, "Failed to decode base64", {})));
        }
    };

    // Generate a unique filename based on the current timestamp
    let uploaded_name = format!("{:?}.jpg", Uuid::new_v4());

    // Create the file in the "uploads" directory
    let upload_path = format!("src/uploads/products/{}", uploaded_name);
    let mut upload_file = match File::create(&upload_path) {
        Ok(file) => file,
        Err(err) => {
            // Handle file creation error
            eprintln!("Failed to create file: {:?}", err);
            return Err(HttpResponse::BadRequest().json(bad_request_response({}, "Failed to create file", {})));
        }
    };

    // Write the decoded data to the file
    if let Err(err) = upload_file.write_all(&decoded_data) {
        // Handle file write error
        eprintln!("Failed to write to file: {:?}", err);
        return Err(HttpResponse::BadRequest().json(bad_request_response({}, "Failed to write to file", {})));
    }
    return Ok(uploaded_name)
}

