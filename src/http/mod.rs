// will be the file the main looks for
// pub mod method;
// pub mod request;

// makes other modules visible

// use statement makes request available simpler
pub use method::Method;
pub use query_string::{QueryString, Value as QueryStringValue};
pub use request::ParseError;
pub use request::Request;
pub use response::Response;
pub use status_code::StatusCode;

pub mod method;
pub mod query_string;
pub mod request;
pub mod response;
pub mod status_code;
