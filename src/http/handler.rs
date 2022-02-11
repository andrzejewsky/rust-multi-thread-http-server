use super::Request;
use super::Response;

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;
}