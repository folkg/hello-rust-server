use std::{
    eprintln,
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
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
        // TODO: do we want to create a `build` method instead that returns a result? What would we do with it?
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
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Job;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
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
