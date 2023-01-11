use std::{
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {id} got job. Executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker id {} disconnected. Shutting down.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for w in self.workers.iter_mut() {
            if let Some(thread) = w.thread.take() {
                println!("Stopping worker {}.", w.id);
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The new function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (tx, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers: Vec<_> = (0..size)
            .map(|id| Worker::new(id, Arc::clone(&receiver)))
            .collect();
        ThreadPool {
            workers,
            sender: Some(tx),
        }
    }

    pub fn execute<F>(&self, f: F)
    // Copied from thread::spawn()
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}
