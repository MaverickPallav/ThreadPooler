mod thread_pool;
mod worker;
mod task;
mod scheduling;
mod monitoring;

use thread_pool::ThreadPool;
use scheduling::{PriorityScheduling, RoundRobinScheduling};
use monitoring::Monitoring;
use std::sync::{Arc, Mutex};
use env_logger;

fn main() {
    // Initialize logging
    env_logger::init();

    // Create a thread pool with round-robin scheduling strategy
    let pool = ThreadPool::new(4, Box::new(RoundRobinScheduling::new()));

    // Wrap the ThreadPool in an Arc and Mutex
    let pool = Arc::new(Mutex::new(pool));

    // Submit some example tasks
    for i in 1..=4 {
        let task_name = format!("Task-{}", i);
        let pool = Arc::clone(&pool);

        log::info!("Task name {}.", task_name);
        std::thread::spawn(move || {
            let pool = pool.lock().unwrap();
            pool.submit_task(move || {
                Monitoring::log_task_start(&task_name);
                std::thread::sleep(std::time::Duration::from_secs(1)); // Simulate work
                Monitoring::log_task_complete(&task_name);
            });
        });
    }

    // Simulate monitoring system load
    {
        let mut pool = pool.lock().unwrap();
        pool.monitor_system_load();
    }
}
