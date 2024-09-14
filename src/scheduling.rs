use std::sync::{Arc, Mutex, mpsc};
use std::cmp::Ordering; // For priority comparison

// Define a struct for a job with a priority and the actual closure (task)
pub struct Job {
    pub priority: usize,
    pub task: Box<dyn FnOnce() + Send + 'static>,
}

// Implement Clone for Job manually
impl Clone for Job {
    fn clone(&self) -> Self {
        // You cannot clone a Box<dyn FnOnce()>, so we handle this by ignoring `task` for cloning
        Job {
            priority: self.priority,
            task: Box::new(|| ()), // Placeholder for cloning
        }
    }
}

// Implement a simple comparison for priority-based scheduling
impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Higher priority comes first
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Job {}

pub trait SchedulingStrategy: Send {
    fn schedule(&mut self, task_queue: &Vec<Job>) -> Option<Job>;
}


// Priority-based scheduling
pub struct PriorityScheduling;

impl SchedulingStrategy for PriorityScheduling {
    fn schedule(&mut self, task_queue: &Vec<Job>) -> Option<Job> {
        task_queue.iter().max_by_key(|job| job.priority).cloned() // Schedule by max priority
    }
}

// Round-robin scheduling
pub struct RoundRobinScheduling {
    current_index: usize,
}

impl RoundRobinScheduling {
    pub fn new() -> Self {
        RoundRobinScheduling { current_index: 0 }
    }
}

impl SchedulingStrategy for RoundRobinScheduling {
    fn schedule(&mut self, task_queue: &Vec<Job>) -> Option<Job> {
        if task_queue.is_empty() {
            None
        } else {
            let job = task_queue[self.current_index].clone();
            self.current_index = (self.current_index + 1) % task_queue.len();
            Some(job)
        }
    }
}


