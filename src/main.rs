mod create_responses;
mod handler;
mod response;
mod router;
mod routes;

use crate::handler::*;

fn main() {
    println!("\r\n\r\n\r\n");

    let http_answer: String = handle_request("GET /time HTTP/1.1");
    println!("{}", http_answer);

    let broken_http_answer: String = handle_request("brokenone");
    println!("{}", broken_http_answer);
}
