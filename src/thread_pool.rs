// src/thread_pool.rs

use crate::scheduling::{SchedulingStrategy, PriorityScheduling, RoundRobinScheduling};
use crate::worker::Worker;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
    receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    scheduling_strategy: Box<dyn SchedulingStrategy + Send>,
}

impl ThreadPool {
    pub fn new(num_threads: usize, strategy: Box<dyn SchedulingStrategy + Send>) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender,
            receiver,
            scheduling_strategy: strategy,
        }
    }

    pub fn submit_task<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.sender.send(job).unwrap();
        log::info!("Task submitted.");
    }

    pub fn monitor_system_load(&mut self) {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        log::info!("Current CPU usage: {}%", cpu_usage);

        if cpu_usage > 80.0 {
            self.add_worker();
        } else if cpu_usage < 30.0 && self.workers.len() > 1 {
            self.remove_worker();
        }
    }

    fn add_worker(&mut self) {
        let id = self.workers.len();
        let worker = Worker::new(id, Arc::clone(&self.receiver));
        self.workers.push(worker);
    }

    fn remove_worker(&mut self) {
        if let Some(mut worker) = self.workers.pop() {
            worker.join();
        }
    }
}
