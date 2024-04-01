use super::status_code::StatusCode;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // dyn is dynamic dispatch, we are using a trait object so we can use different types
    // all implementors have an implementation of the Write trait, it is called dyn because it is dynamic implementation
    // at runtime, the compiler will figure out which implementation to use, it is a downside because it is slower
    // another is static dispatch, it is faster because it is determined at compile time, we change dyn with impl
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
