#![allow(dead_code)]

use http::Method;
use http::Request;
use server::Server;
use std::default;
use std::env;
use website_handler::WebsiteHandler;

// the same as the compiler pasting all code here, needs t have the same name as the file
mod http;
mod server;
mod website_handler;

fn main() {
    // Create an enum with different methods
    // let get = Method::GET("abcd".to_string();
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR")); // macro that gets the value of a env variable
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    let mut server = Server::new("127.0.0.1:8080");

    server.run(WebsiteHandler::new(public_path));
}
