pub fn route(http_method: &str, http_path: &str) {
    if http_method == "GET" {
        if http_path == "/hello" {
            println!("Hello command issued");
        }
    }
}
