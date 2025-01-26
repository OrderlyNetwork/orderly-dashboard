use std::time::Duration;
use sysinfo::System;

pub struct MemoryMonitor {
    threshold_mb: f64,         // Memory threshold (MB)
    warning_threshold_mb: f64, // Warning threshold (MB)
    sample_interval: Duration, // Sampling interval
}

impl MemoryMonitor {
    pub fn new(threshold_mb: f64, warning_threshold_mb: f64, sample_interval: Duration) -> Self {
        Self {
            threshold_mb,
            warning_threshold_mb,
            sample_interval,
        }
    }

    pub async fn start_monitoring(&self) {
        let mut sys = System::new_all();
        let pid = std::process::id();
        let mut peak_memory = 0.0f64;

        loop {
            sys.refresh_all();

            if let Some(process) = sys.process((pid as usize).into()) {
                let memory_mb = process.memory() as f64 / 1024.0 / 1024.0;
                peak_memory = peak_memory.max(memory_mb);
                let cpu_usage = process.cpu_usage();

                tracing::info!(
                    "Memory stats: current={:.2}MB, peak={:.2}MB, virtual={:.2}MB, cpu_usage={:.2}%",
                    memory_mb,
                    peak_memory,
                    process.virtual_memory() as f64 / 1024.0 / 1024.0,
                    cpu_usage
                );

                // Check if memory usage exceeds the warning threshold
                if memory_mb >= self.warning_threshold_mb {
                    tracing::warn!(
                        "Memory usage is approaching the threshold! Current: {:.2} MB, Warning threshold: {:.2} MB",
                        memory_mb,
                        self.warning_threshold_mb
                    );
                }

                // Check if memory usage exceeds the danger threshold
                if memory_mb >= self.threshold_mb {
                    tracing::warn!(
                        "Memory usage exceeds threshold! Current: {:.2} MB, Threshold: {:.2} MB",
                        memory_mb,
                        self.threshold_mb
                    );

                    // You can add alert logic here, for example:
                    // 1. Send alert email
                    // 2. Trigger process restart
                    // 3. Clear cache
                    self.handle_memory_overflow().await;
                }
            }

            tokio::time::sleep(self.sample_interval).await;
        }
    }

    async fn handle_memory_overflow(&self) {
        // mplement the logic to handle memory overflow
        // For example:
        // 1. Clear cache
        // 2. Send alert
        // 3. Restart service
    }
}

// // Example usage
// #[tokio::main]
// async fn main() {
//     // Set 1GB as the danger threshold, 800MB as the warning threshold
//     let monitor = MemoryMonitor::new(
//         1024.0,  // 1GB
//         800.0,   // 800MB
//         Duration::from_secs(60)
//     );

//     // Start monitoring
//     tokio::spawn(async move {
//         monitor.start_monitoring().await;
//     });

//     // Your main program logic
//     // ...
// }
