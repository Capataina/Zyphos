use crate::{
    response::HttpResponse,
    routes::{hello, time},
};

pub fn route(http_method: &str, http_path: &str) -> HttpResponse {
    if http_method == "GET" {
        if http_path == "/hello" {
            return hello::handle();
        } else if http_path == "/time" {
            return time::handle();
        }
    }

    HttpResponse {
        status_code: 404,
        status_text: "Not Found".to_string(),
        body: "404 Not Found".to_string(),
    }
}
