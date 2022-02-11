use std::thread;
use std::sync::{ Arc, Mutex, mpsc };
use super::Message;

pub struct Worker {
  id: usize,
  thread: Option<thread::JoinHandle<()>>
}

impl Worker {
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      let message = receiver.lock().unwrap().recv().unwrap();

      match message {
        Message::NewJob(job) => {
          println!("Worker {}", id);
          job();
        },
        Message::Terminate => {
          println!("Worker {} terminating...", id);
          break;
        }
      }
    });

    Worker { id, thread: Some(thread) }
  }

  pub fn id(&self) -> usize {
    self.id
  }

  pub fn join(&mut self) {
    if let Some(thread) = self.thread.take() {
      thread.join().unwrap();
    }
  }
}