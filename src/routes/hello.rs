use crate::{create_responses::create_text_response, response::HttpResponse};

pub fn handle() -> HttpResponse {
    create_text_response("Hello World!".to_string())
}
