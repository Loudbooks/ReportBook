use sysinfo::System;
use crate::reportbook::process::Process;

pub fn gather_processes(sys: &System) -> Vec<Process> {
    let username = whoami::username();
    let mut processes: Vec<Process> = Vec::new();

    for (_pid, process) in sys.processes() {
        match process.cwd() {
            None => {}
            Some(_) => {
                let amount = if processes.iter().any(|p| p.name == process.name().to_str().unwrap()) {
                    let new_process = processes
                        .iter_mut()
                        .find(|p| p.name == process.name().to_str().unwrap())
                        .unwrap();
                    new_process.amount += 1;
                    new_process.memory += process.memory() as f64;

                    continue;
                } else {
                    1
                };

                let hashtags = '#'.to_string().repeat(username.len());

                let path = if cfg!(target_os = "windows") {
                    process.cwd().unwrap().to_str().unwrap().replace(
                        format!("\\{}", username).as_str(),
                        format!("\\{}", hashtags.as_str()).as_str(),
                    )
                } else {
                    process.cwd().unwrap().to_str().unwrap().replace(
                        format!("/{}", username).as_str(),
                        format!("/{}", hashtags.as_str()).as_str(),
                    )
                };

                let process = Process {
                    name: process.name().to_str().unwrap().to_string(),
                    path,
                    memory: process.memory() as f64,
                    amount,
                };

                processes.push(process);
            }
        }
    }
    processes.sort_by_key(|p| p.memory as i64);
    processes.reverse();

    processes
}
