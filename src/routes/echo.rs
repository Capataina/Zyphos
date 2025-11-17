use crate::{create_responses::create_text_response, response::HttpResponse};

pub fn handle(text: &str) -> HttpResponse {
    create_text_response(text.to_string())
}
