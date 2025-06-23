mod handler;
mod router;

use crate::handler::*;

fn main() {
    handle_request("GET /hello HTTP/1.1");
}
