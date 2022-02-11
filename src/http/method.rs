use std::str::FromStr;
use std::fmt::{ Display, Debug, Formatter, Result as FmtResult };
use super::error::MethodError;

#[derive(Debug)]
pub enum Method {
  GET,
  POST,
  PUT,
  DELETE
}

impl FromStr for Method {
  type Err = MethodError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Self::GET),
      "POST" => Ok(Self::POST),
      "PUT" => Ok(Self::PUT),
      "DELETE" => Ok(Self::DELETE),
      _ => Err(MethodError),
    }
  }
}

impl Display for Method {
  fn fmt(&self, f: &mut Formatter) -> FmtResult {
    write!(f, "{:?}", self)
  }
}