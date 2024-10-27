use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use crate::concurrency_primitives::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>
}

pub type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(threads_number: usize) -> ThreadPool {
        assert!(threads_number > 0);
        let mut workers: Vec<Worker> = Vec::with_capacity(threads_number);

        let (sender, receiver) = channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..threads_number {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        };

        ThreadPool {workers, sender}
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce(),
        F: Send + 'static
    {
        let job = Box::new(f);

        &self.sender.send(job).unwrap();
    }
}




