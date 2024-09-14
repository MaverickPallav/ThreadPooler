use std::panic::{catch_unwind, AssertUnwindSafe}; // Add this import
use std::sync::{Arc, Mutex, mpsc};
use log::{info, error}; // For logging

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = std::thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => {
                    info!("Worker {} got a job; executing.", id);
                    // Wrap the job in AssertUnwindSafe to allow catch_unwind
                    if let Err(e) = catch_unwind(AssertUnwindSafe(|| {
                        job();
                    })) {
                        error!("Worker {} panicked during job execution: {:?}", id, e);
                    }
                }
                Err(_) => {
                    error!("Worker {} disconnected; shutting down.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }

    pub fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
