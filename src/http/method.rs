use std::{fmt::Display, str::FromStr};

// incremental values, however, if we set a value everything else will increment from there
#[derive(Debug)]
pub enum Method {
    // can be associated to different types of data
    // GET(String), // 0
    // DELETE(u64), // 1
    GET,    // 0
    DELETE, // 1
    POST = 5,
    PUT, // 6
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}
// these are their value representations in memory
// after we set an associated ttpe, each variant in memory will use the larges possible case

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::DELETE => write!(f, "DELETE"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::HEAD => write!(f, "HEAD"),
            Method::CONNECT => write!(f, "CONNECT"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::TRACE => write!(f, "TRACE"),
            Method::PATCH => write!(f, "PATCH"),
        }
    }
}

//when implementing this, the parse() will work for us
impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "DELETE" => Ok(Method::DELETE),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "HEAD" => Ok(Method::HEAD),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(MethodError),
        }
    }
}
pub struct MethodError;
