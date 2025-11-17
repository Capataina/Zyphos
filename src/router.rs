use crate::{
    create_responses::create_error_response,
    response::HttpResponse,
    routes::{echo, hello, time},
};

pub fn route(http_method: &str, http_path: &str) -> HttpResponse {
    if http_method == "GET" {
        if http_path == "/hello" {
            return hello::handle();
        } else if http_path == "/time" {
            return time::handle();
        }

        if let Some(text) = http_path.strip_prefix("/echo/") {
            return echo::handle(text);
        }
    }

    create_error_response(
        404,
        "Not Found".to_string(),
        "The entered path doesn't exist.".to_string(),
    )
}
