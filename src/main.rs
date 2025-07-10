mod create_responses;
mod handler;
mod response;
mod router;
mod routes;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::panic::{self, AssertUnwindSafe};
use std::thread;

use crate::handler::*;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream
        .read(&mut buffer)
        .expect("Failed to read from connection.");
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let request = request.trim_matches(char::from(0)).trim();

    println!("Received request: {request}");

    let http_response = handle_request(request);
    let response_bytes = http_response.as_bytes();
    stream
        .write_all(response_bytes)
        .expect("Failed to send the response.");

    stream.flush().expect("Failed to flush stream.");

    println!("Response sent successfully.");
}

fn main() {
    let server_address = "localhost:3000";
    let listener = TcpListener::bind(server_address).expect("Failed to bind to address");
    println!("The server has started!");

    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                println!("Received stream, spawning thread.");
                thread::spawn(move || {
                    let connection_id = 1;
                    println!("[INFO] [Conn#{connection_id}] Starting connection handler.");

                    let result = panic::catch_unwind(AssertUnwindSafe(|| {
                        handle_connection(stream);
                    }));

                    match result {
                        Ok(_) => {
                            println!(
                                "[INFO] [Conn#{connection_id}] Connection handled successfully"
                            );
                        }
                        Err(panic_info) => {
                            println!(
                                "[ERROR] [Conn#{connection_id}] Thread panicked during connection handling"
                            );
                            // Optionally try to extract panic message
                            if let Some(message) = panic_info.downcast_ref::<&str>() {
                                println!("[ERROR] Panic message: {message}");
                            } else if let Some(message) = panic_info.downcast_ref::<String>() {
                                println!("[ERROR] Panic message: {message}");
                            } else {
                                println!(
                                    "[ERROR] Panic occurred but message could not be extracted"
                                );
                            }
                        }
                    }

                    println!("[INFO] [Conn#{connection_id}] Thread ending");
                });
            }
            Err(error) => {
                println!("Connection failed: {error}");
                println!("Error type: {:?}", error.kind());
            }
        }
    }
}
