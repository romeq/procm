mod stats;

use clap::Command;
use owo_colors::OwoColorize;

fn main() {
    let matches = Command::new("procm")
        .about("Show program's used resource")
        .subcommand_required(true)
        .subcommand(
            Command::new("show")
                .about("Show process statistics")
                .arg(clap::arg!(pid: <pid>).value_parser(clap::value_parser!(usize))),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("show", show_matches)) => {
            let pid = *show_matches.get_one::<usize>("pid").unwrap();
            if stats::show_resources_for_pid(pid).is_none() {
                println!("{}: {}", "Error".red().bold(), "No such process found")
            }
        }
        _ => unreachable!(),
    }
}
