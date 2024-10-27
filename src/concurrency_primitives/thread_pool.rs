use crate::concurrency_primitives::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>
}

impl ThreadPool {
    fn new(threads_number: usize) -> ThreadPool {
        assert!(threads_number > 0);
        let mut workers: Vec<Worker> = Vec::with_capacity(threads_number);

        for id in 0..threads_number {
            workers.push(Worker::new(id))
        };

        ThreadPool {workers}
    }

    fn execute<F>(f: F)
    where
        F: FnOnce(),
        F: Send + 'static
    {

    }
}


