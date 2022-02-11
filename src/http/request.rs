use std::convert::TryFrom;
use std::str;
use std::fmt::{ Debug };
use super::error::ParseError;
use super::method::Method;
use super::query_string::QueryString;


#[derive(Debug)]
pub struct Request<'a> {
  path: &'a str,
  query: Option<QueryString<'a>>,
  method: Method,
}

impl<'a> Request<'a> {
  pub fn path(&self) -> &str {
    self.path
  }

  pub fn query(&self) -> Option<&QueryString> {
    // Converts from &Option<T> to Option<&T>
    self.query.as_ref()
  }

  pub fn method(&self) -> &Method {
    &self.method
  }
}

fn next_part(request: &str) -> Option<(&str, &str)> {
  for (i, c) in request.chars().enumerate() {
    if c == ' ' || c == '\r' {
      return Some((&request[..i], &request[i + 1..]));
    }
  }

  None
}

fn resolve_query_string(path: &str) -> Option<QueryString> {
  let mut query = None;

  if let Some(i) = path.find('?') {
    query = Some(QueryString::from(&path[i + 1..]));
  }

  query
}

impl<'a> TryFrom<&'a [u8]> for Request<'a> {
  type Error = ParseError;

  fn try_from(buffer: &'a [u8]) -> Result<Self, Self::Error> {
    /*
      question mark is equivalent of 
        let request = match str::from_utf8(&buffer[..]) {
          Ok(i) => i,
          Err(e) => e // convert using Into::into
        }
     */

    let request = str::from_utf8(&buffer[..])?;

    let (method, request) = next_part(request).ok_or(ParseError::InvalidRequest)?;
    let (path, request) = next_part(request).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = next_part(request).ok_or(ParseError::InvalidRequest)?; 

    if protocol != "HTTP/1.1" {
      return Err(ParseError::InvalidProtocol);
    }

    // converts the string into the type Method, by using FromStr trait
    let method: Method = method.parse()?;
    let query = resolve_query_string(path);

    Ok(Self { path, query, method })
  }
}