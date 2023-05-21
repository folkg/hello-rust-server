use std::{eprintln, thread};

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if `size` is 0
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id))
        }

        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize) -> Worker {
        let builder = thread::Builder::new();

        let thread = match builder.spawn(|| {}) {
            Ok(thread) => thread,
            Err(error) => {
                eprintln!("Failed to spawn thread: {:?}", error);
                return Worker { id, thread: None };
            }
        };

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
