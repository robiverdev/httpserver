use std::convert::TryFrom;
use std::fmt::Error;

use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>,
    pub method: Method,
}

impl TryFrom<&[u8]> for Request {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
