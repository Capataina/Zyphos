use std::collections::HashMap;

use chrono::Utc;

use crate::response::HttpResponse;

pub fn get_http_date() -> String {
    Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

pub fn create_text_response(body: String) -> HttpResponse {
    let mut response_headers: HashMap<String, String> = HashMap::new();

    response_headers.insert("Content-Type".to_string(), "text/plain".to_string());
    response_headers.insert("Content-Length".to_string(), body.len().to_string());
    response_headers.insert("Connection".to_string(), "close".to_string());
    response_headers.insert("Date".to_string(), get_http_date());

    HttpResponse {
        status_code: 200,
        status_text: "OK".to_string(),
        headers: response_headers,
        body,
    }
}

pub fn create_error_response(
    status_code: i32,
    status_text: String,
    status_body: String,
) -> HttpResponse {
    let mut response_headers: HashMap<String, String> = HashMap::new();

    response_headers.insert("Content-Type".to_string(), "text/plain".to_string());
    response_headers.insert("Content-Length".to_string(), status_body.len().to_string());
    response_headers.insert("Connection".to_string(), "close".to_string());
    response_headers.insert("Date".to_string(), get_http_date());

    HttpResponse {
        status_code,
        status_text,
        headers: response_headers,
        body: status_body,
    }
}
