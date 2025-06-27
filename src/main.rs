mod handler;
mod response;
mod router;
mod routes;

use crate::handler::*;

fn main() {
    let http_answer: String = handle_request("GET /time HTTP/1.1");
    println!("{}", http_answer);
}
