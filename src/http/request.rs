use super::method::Method;
use super::method::MethodError;
use std::convert::From;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

impl<'buf> Display for Request<'buf> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        let query_string = match &self.query_string {
            Some(qs) => qs.to_string(),
            None => "-".to_string(),
        };
        write!(
            f,
            "Path: {}\nQuery String: {}\nMethod: {}",
            self.path, query_string, self.method
        )
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buffer: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path_and_query, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let pivot_option = path_and_query.find('?');
        let path = match pivot_option {
            None => path_and_query,
            Some(i) => &path_and_query[..i],
        };
        let query_string: Option<&str> = pivot_option.map(|i| &path_and_query[i + 1..]);
        let method: Method = method.parse()?;

        let request = Request {
            path,
            query_string,
            method,
        };

        Ok(request)
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
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
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
