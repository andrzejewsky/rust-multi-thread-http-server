use std::io::{ Write, Result as IoResult };
use super::status_code::StatusCode;

#[derive(Debug)]
pub struct Response {
  status: StatusCode,
  body: Option<String>,
}

impl Response {
  pub fn new(status: StatusCode, body: Option<String>) -> Self {
    Response { status, body }
  }

  pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
    let body = match &self.body {
      Some(b) => b,
      None => "",
    };

    write!(
      stream,
      "HTTP/1.1 {} {}\r\n\r\n{}",
      self.status,
      self.status.reason_phrase(),
      body
    )
  }
}