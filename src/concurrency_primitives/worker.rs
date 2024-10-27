use std::thread::JoinHandle;

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let thread = std::thread::spawn(|| {

        });

        Worker {id, thread}
    }
}