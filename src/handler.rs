use crate::{
    response::{HttpResponse, format_response},
    router::route,
};

pub fn handle_request(raw_request: &str) -> String {
    let parsed_request: Vec<&str> = raw_request.split(" ").collect();

    if parsed_request.len() <= 1 {
        let bad_request = HttpResponse {
            status_code: 400,
            status_text: "Bad Request".to_string(),
            body: "The request was missing either the method or the path.".to_string(),
        };

        return format_response(bad_request);
    }

    let http_method = parsed_request[0];
    let http_path = parsed_request[1];

    let http_response = route(http_method, http_path);

    format_response(http_response)
}
