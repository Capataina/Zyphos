use std::collections::HashMap;

pub struct HttpResponse {
    pub status_code: i32,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub fn format_response(response: HttpResponse) -> String {
    let status_line = format!(
        "HTTP/1.1 {} {}\r\n",
        response.status_code, response.status_text
    );
    let mut headers_section = String::new();
    let important_headers = [
        "Content-Type".to_string(),
        "Content-Length".to_string(),
        "Connection".to_string(),
        "Date".to_string(),
        "Server".to_string(),
    ];

    for header in &important_headers {
        if let Some(value) = response.headers.get(header) {
            headers_section.push_str(&format!("{}: {}\r\n", header, value));
        }
    }

    for (key, value) in response.headers.into_iter() {
        if !important_headers.contains(&key) {
            headers_section.push_str(&format!("{}: {}\r\n", key, value))
        }
    }
    format!(
        "{}{}\r\n{}\r\n\r\n\r\n",
        status_line, headers_section, response.body
    )
}
