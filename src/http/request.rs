// super is go one level up
use super::method::{Method, MethodError};
use super::{QueryString, QueryStringValue};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

// Rust does not support null values, we do get Option<T> which is an enum that represents absence of a value
#[derive(Debug)]
pub struct Request<'buf> {
    // lifetimes have ' in front of them, they are used to specify the scope of a reference

    // since all will come from buffer, it is not optimal to store a string, we store a reference
    // for this we need lifetimes because the buffer we receive will be dropped after the function ends
    path: &'buf str,
    // Can be None or a Some(String) value, the value is a string which replaces the generic T
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

// name getter after name with no get_ prefix
impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

impl<'buf> Display for Request<'buf> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {}", self.method, self.path)
    }
}

// To use a lifetime we need to define it in the impl
// Implement the trait for the struct
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(value: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(value)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        println!("Protocol: {}", protocol);
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..].to_string());
        //         path = &path[..i];
        //     }
        //     None => {}
        // }

        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..].to_string());
        //     path = &path[..i];
        // }

        // if let allows normal statements to be used like in a match
        if let Some(i) = path.find('?') {
            // look at result of find and try to match it to variant provided
            query_string = Option::Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path: path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars();
    // loop {
    // let item = iter.next();
    // match item {
    //     Some(c) => {
    //     }
    //     None => return None,
    // }
    // }

    // if we call enumerate on an iterator, we can get a touple with indices
    for (i, c) in request.chars().enumerate() {
        // since space is one character, skipping is safe
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

// Create new trait
// trait Encrypt {
//     fn encrypt(&self) -> Self;
// }

// impl Encrypt for &[u8] {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }

// impl Encrypt for String {
//     fn encrypt(&self) -> Self {
//         unimplemented!()
//     }
// }
