use owo_colors::OwoColorize;
use size_format::SizeFormatterSI;
use sysinfo::{Pid, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt};

pub fn show_resources_for_pid(pid: usize) -> Option<()> {
    let mut sys = System::new();

    sys.refresh_specifics(
        RefreshKind::new()
            .with_processes(ProcessRefreshKind::everything())
            .with_memory(),
    );

    let process = sys.process(Pid::from(pid))?;
    let disk_usage = process.disk_usage();

    println!(
        "Memory: \t{}",
        print_size_formatter(process.memory(), Some(sys.total_memory()))
    );
    println!("CPU (%): \t{:?}", process.cpu_usage().green());
    println!(
        "Disk (w, r): \t{}, {}",
        SizeFormatterSI::new(disk_usage.total_written_bytes).green(),
        SizeFormatterSI::new(disk_usage.total_read_bytes).green(),
    );

    Some(())
}

fn print_size_formatter(primary: u64, secondary: Option<u64>) -> String {
    let size_formatter = SizeFormatterSI::new(primary);
    if secondary.is_none() {
        return size_formatter.to_string();
    }

    let memory_usage = (primary / secondary.unwrap()) * 100;
    let size_formatter_total = SizeFormatterSI::new(secondary.unwrap());

    format!(
        "{} (out of {})",
        if memory_usage < 20 {
            size_formatter.green().to_string()
        } else if memory_usage > 80 {
            size_formatter.red().to_string()
        } else {
            size_formatter.yellow().to_string()
        },
        size_formatter_total,
    )
}
