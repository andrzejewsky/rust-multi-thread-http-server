use crate::http::{ Handler, Request, Response, StatusCode, Method };

pub struct HomeHandler;

impl HomeHandler {
  pub fn new() -> HomeHandler {
    HomeHandler {}
  }
}

impl Handler for HomeHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        _ => {
          if let Some(query) = request.query() {
            println!("HomeHandler query: {:?}", query.get("test"));
          }

          Response::new(StatusCode::Ok, Some("ok".to_string()))
        },
      },
      _ => Response::new(StatusCode::NotFound, None)
    }
  }
}
