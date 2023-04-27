use owo_colors::OwoColorize;
use size_format::SizeFormatterSI;
use sysinfo::{Pid, ProcessExt, ProcessRefreshKind, RefreshKind, System, SystemExt, CpuRefreshKind};

pub fn show_resources_for_pid(pid: usize) -> Option<()> {
    let mut sys = System::new();

    let cpu_refresh_kind = RefreshKind::new()
        .with_processes(ProcessRefreshKind::everything())
        .with_cpu(CpuRefreshKind::everything());

    // CPU has to be refreshed twice to get accurate result
    sys.refresh_specifics(cpu_refresh_kind.with_memory());
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_specifics(cpu_refresh_kind);

    let process = sys.process(Pid::from(pid))?;
    let cpu_count = sys.cpus().len();
    let cpu_usage = process.cpu_usage();

    println!("Memory: \t{}",
        print_size_formatter(process.memory(), Some(sys.total_memory())));

    println!("CPU (rel/abs): \t{:.4}% / {:.4}%", (cpu_usage / cpu_count as f32).green(), cpu_usage.green());

    let disk_usage = process.disk_usage();
    println!("Disk (w, r): \t{}, {}",
        SizeFormatterSI::new(disk_usage.total_written_bytes).green(),
        SizeFormatterSI::new(disk_usage.total_read_bytes).green());

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
