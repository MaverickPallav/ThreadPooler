use log::{info, error};
use sysinfo::{CpuExt, System, SystemExt};

pub struct Monitoring;

impl Monitoring {
    pub fn monitor_system_load() {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        info!("Current CPU usage: {}%", cpu_usage);
        
        if cpu_usage > 80.0 {
            info!("CPU usage is high. Consider adding more workers.");
        } else if cpu_usage < 30.0 {
            info!("CPU usage is low. Consider reducing the number of workers.");
        }
    }

    pub fn log_task_start(task_name: &str) {
        info!("Task {} started.", task_name);
    }

    pub fn log_task_complete(task_name: &str) {
        info!("Task {} completed.", task_name);
    }
}
