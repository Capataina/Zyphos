use crate::{create_responses::create_error_response, response::format_response, router::route};

pub fn handle_request(raw_request: &str) -> String {
    let request_lines: Vec<&str> = raw_request.lines().collect();

    let request_line = if request_lines.is_empty() {
        ""
    } else {
        request_lines[0]
    };

    let parsed_request: Vec<&str> = request_line.split_whitespace().collect();

    if parsed_request.len() != 3 {
        let bad_request = create_error_response(
            400,
            "Bad Request".to_string(),
            "Invalid HTTP request line format. Expected: METHOD PATH HTTP/VERSION.".to_string(),
        );

        return format_response(bad_request);
    }

    let http_method = parsed_request[0];
    let http_path = parsed_request[1];
    let http_version = parsed_request[2];

    if !http_version.starts_with("HTTP/") {
        let bad_request = create_error_response(
            400,
            "Bad Request".to_string(),
            "The request is not a valid \"HTTP\" request.".to_string(),
        );

        return format_response(bad_request);
    }

    let http_response = route(http_method, http_path);

    format_response(http_response)
}
