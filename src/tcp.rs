use std::net::{ TcpListener, TcpStream };
use std::io::{ Read };
use crate::http::{ Request, Response, StatusCode, Handler };
use crate::handlers::{ HomeHandler };
use crate::processing::ThreadPool;

pub struct Server {
  addr: String
}

impl Server {
  pub fn new(addr: String) -> Server {
    Server { addr }
  }

  pub fn listen(&self) {
    let listener = TcpListener::bind(self.addr.to_string()).unwrap();

    let pool = ThreadPool::new(4);

    loop {
      match listener.accept() {
        Ok((stream, _)) => {
          pool.execute(|| {
            process_stream(stream)
          });
        },
        Err(e) => println!("Unable to read stream: {}", e)
      }
    }
  }
}

fn process_stream(mut stream: TcpStream) {
  let mut buffer = [0; 1024];

  match stream.read(&mut buffer) {
    Ok(_) => {
      let response = match Request::try_from(&buffer[..]) {
        Ok(request) => {
          println!("path: {}, query: {:?}, method: {:?}", request.path(), request.query(), request.method());
          let mut handler = HomeHandler::new();
          handler.handle_request(&request)
        },
        Err(e) => {
          println!("Unable to parse the request, reason: {}", e);

          Response::new(StatusCode::BadRequest, None)
        }
      };

      if let Err(e) = response.send(&mut stream) {
        println!("Error during sending response: {}", e);
      }
    }
    Err(e) => println!("Unable to handle the request, reason: {}", e)
  };
}