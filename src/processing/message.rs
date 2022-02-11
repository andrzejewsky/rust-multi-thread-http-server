type Job = Box<dyn FnOnce() + Send + 'static>;

pub enum Message {
  NewJob(Job),
  Terminate,
}