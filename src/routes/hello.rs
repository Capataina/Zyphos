use crate::response::HttpResponse;

pub fn handle() -> HttpResponse {
    HttpResponse {
        status_code: 200,
        status_text: "OK".to_string(),
        body: "Hello World!".to_string(),
    }
}
