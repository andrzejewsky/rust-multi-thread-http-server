mod tcp;
mod http;
mod handlers;
mod processing;

fn main() {
  let server = tcp::Server::new("127.0.0.1:7979".to_string());

  server.listen();
}
