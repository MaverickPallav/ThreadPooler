use threadpooler_r::thread_pool::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    let mut pool = ThreadPool::new(3);

    for i in 1..=10 {
        pool.execute(move || {
            println!("Executing Task-{}", i);
            thread::sleep(Duration::from_secs(1));
            println!("Task-{} Completed", i);
        });
    }

    pool.shutdown();
}
