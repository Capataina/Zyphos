pub struct HttpResponse {
    pub status_code: i32,
    pub status_text: String,
    pub body: String,
}

pub fn format_response(response: HttpResponse) -> String {
    format!(
        "HTTP/1.1 {} {} \r\n\r\n {}",
        response.status_code, response.status_text, response.body,
    )
}
