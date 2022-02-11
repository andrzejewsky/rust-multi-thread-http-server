use std::sync::{ Arc, Mutex, mpsc };
use super::Message;
use super::Worker;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)))
    }

    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where 
    F: FnOnce() + Send + 'static
    {
      let job = Box::new(f);
      self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Terminating all workers...");

    for _ in &self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    for worker in &mut self.workers {
      println!("Turn of worker: {}", worker.id());
      worker.join();
    }
  }
}