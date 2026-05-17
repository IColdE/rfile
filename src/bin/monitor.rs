use sysinfo::{System, CpuRefreshKind, RefreshKind, MemoryRefreshKind};
use std::thread;
use std::time::Duration;
use colored::*;

fn main() {
    // Correct way to initialize specifics in sysinfo 0.33
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
    );

    println!("{}", "=== System Monitor (Press Ctrl+C to stop) ===".cyan().bold());

    loop {
        // Refresh system information
        sys.refresh_cpu_all();
        sys.refresh_memory();

        // Clear screen (using ANSI escape codes)
        print!("\x1B[2J\x1B[1;1H");

        println!("{}", "=== System Monitor ===".cyan().bold());
        
        // CPU Usage
        println!("\n{}", "--- CPU Usage ---".yellow());
        for (i, cpu) in sys.cpus().iter().enumerate() {
            let usage = cpu.cpu_usage();
            let bar_len = (usage / 5.0) as usize;
            let bar = "|".repeat(bar_len);
            println!("CPU {:2}: [{:<20}] {:>5.1}%", i, bar.green(), usage);
        }

        // Memory Usage
        println!("\n{}", "--- Memory ---".yellow());
        let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let mem_percent = (used_mem / total_mem) * 100.0;
        
        println!("Used:  {:>6.2} GB", format!("{:.2}", used_mem).red());
        println!("Total: {:>6.2} GB", format!("{:.2}", total_mem).green());
        println!("Usage: {:>6.1}%", format!("{:.1}", mem_percent).bold());

        // Sleep for a second before next refresh
        thread::sleep(Duration::from_secs(1));
    }
}
