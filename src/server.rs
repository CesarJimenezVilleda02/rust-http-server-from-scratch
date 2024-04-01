use crate::http::{response, ParseError, Request, Response, StatusCode}; // crate is root of entire crate
use std::convert::TryFrom; // need to use trait to implement it

use std::io::{Read, Write};
use std::net::TcpListener;

// everything inside a module is private by default
pub struct Server {
    addr: String,
}

// traits can have default implementations
pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

impl Server {
    // Associated function, we use :: to access associated function, similar to static
    // fn new(addr: &str) -> Server {
    //     // Type of struct, then brackets with values
    //     Server {
    //         addr: String::from(addr),
    //     }
    // }

    // Uppercase delf is an alias for the name of the struct
    pub fn new(addr: &str) -> Self {
        // Type of struct, then brackets with values
        Self {
            addr: String::from(addr),
        }
    }

    // We use self as a parameter to create a method, self follows ownership rules
    // self would be destroyed after the method is called, to avoid this we use &self
    pub fn run(&mut self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        // bind returns a Result, we use unwrap to get the value
        // two errors, recoverable and unrecoverable, result is a enum that wraps two generics
        // first generic is the actual value, the second is the error
        // this operation could fail

        //unwrap will look at result, if it is an Ok, it will return the value, if it is an Err, it will panic
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            // check for new connections. returns a Result wrapping a tuple of tcpstream and the address
            // code will hang here until new connection is made
            match listener.accept() {
                // if we want to ignore a value we cna replace it with a _
                Ok((mut stream, _)) => {
                    // buffer to store the data
                    let mut buffer = [0; 1024];
                    // read the data from the stream and store it in the buffer
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Request::try_from(&buffer as &[u8]);
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    // // if we get a request, we print the method and path
                                    // // println!("Request: {}", request);
                                    // dbg!(request);

                                    // Response::new(
                                    //     StatusCode::Ok,
                                    //     Some("<h1>It works!</h1>".to_string()),
                                    // )
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    // println!("Failed to parse a request: {}", e);
                                    // Response::new(StatusCode::BadRequest, None)
                                    handler.handle_bad_request(&e)
                                }
                            }; // slice of buffer

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                } // _ => {} // default case if we dont catch all variantes
            }
        }
    }
}

// a file in rust is treated the same as a module
