use crate::routes::{hello, time};

pub fn route(http_method: &str, http_path: &str) -> String {
    if http_method == "GET" {
        if http_path == "/hello" {
            return hello::handle();
        } else if http_path == "/time" {
            return time::handle();
        }
    }

    "404 Not Found".to_string()
}
