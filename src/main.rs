extern crate alfred;

use std::io;
use clap::{App, Arg};
use sysinfo::{Process, ProcessExt, System, SystemExt, RefreshKind, ProcessRefreshKind};

fn main() {
    // access metadata from cargo package http://stackoverflow.com/a/27841363/745121
    let args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("query").required(true).index(1))
        .get_matches();


    let query = args.value_of("query").unwrap();

    let s = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));

    workflow_output(s.processes_by_name(query).collect());
}

fn workflow_output(processes: Vec<&Process>) {
    let items = processes
        .into_iter()
        .filter(|process| !process.name().contains("Helper"))
        .map(|process| {
            let path = process.root().to_string_lossy().replace(".app/Contents/MacOS", ".app");
            alfred::ItemBuilder::new(process.name())
                .arg(process.pid().to_string())
                .icon_file(path.to_string())
                .subtitle(path)
                .into_item()
        })
        .collect::<Vec<alfred::Item>>();

    alfred::json::Builder::with_items(&items)
        .write(io::stdout())
        .expect("Couldn't write items to Alfred");
}

