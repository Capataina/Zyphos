use chrono;

use crate::{create_responses::create_text_response, response::HttpResponse};

pub fn handle() -> HttpResponse {
    create_text_response(chrono::Local::now().format("%d/%m/%Y %T").to_string())
}
