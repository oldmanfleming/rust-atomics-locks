use std::{collections::VecDeque, sync::{Condvar, Mutex}};

pub struct Channel<T> {
  queue: Mutex<VecDeque<T>>,
  item_ready: Condvar,
}

impl<T> Channel<T> {
  pub fn new() -> Self {
    Self {
      queue: Mutex::new(VecDeque::new()),
      item_ready: Condvar::new(),
    }
  }

  pub fn send(&self, message: T) {
    self.queue.lock().unwrap().push_back(message);
    self.item_ready.notify_one();
  }

  pub fn receive(&self) -> T {
    let mut queue = self.queue.lock().unwrap();
    loop {
      if let Some(message) = queue.pop_front() {
        return message;
      }
      // The condvar will unlock the mutex while waiting
      // and will lock it again when it wakes up
      queue = self.item_ready.wait(queue).unwrap();
    }
  }
}