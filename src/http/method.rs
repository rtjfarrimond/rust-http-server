use std::fmt::{Display, Error as FmtError, Formatter};
use std::str::FromStr;

pub enum Method {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        let s = match self {
            Self::GET => "GET",
            Self::HEAD => "HEAD",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::CONNECT => "CONNECT",
            Self::OPTIONS => "OPTIONS",
            Self::TRACE => "TRACE",
            Self::PATCH => "PATCH",
        };
        write!(f, "{}", s.to_string())
    }
}

pub struct MethodError;

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "HEAD" => Ok(Self::HEAD),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "TRACE" => Ok(Self::TRACE),
            "PATCH" => Ok(Self::PATCH),
            _ => Err(MethodError),
        }
    }
}
