use crate::{response::format_response, router::route};

pub fn handle_request(raw_request: &str) -> String {
    let parsed_request: Vec<&str> = raw_request.split(" ").collect();

    let http_method = parsed_request[0];
    let http_path = parsed_request[1];

    let http_response = route(http_method, http_path);

    format_response(http_response)
}
