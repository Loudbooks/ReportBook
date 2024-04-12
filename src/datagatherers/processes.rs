use sysinfo::System;
use crate::process::Process;

pub fn gather_processes(sys: &System) -> Vec<Process> {
    let mut processes: Vec<Process> = Vec::new();

    for (pid, process) in sys.processes() {
        match process.cwd() {
            None => {}
            Some(_) => {
                let amount = if processes.iter().any(|p| p.name == process.name()) {
                    let new_process = processes.iter_mut().find(|p| p.name == process.name()).unwrap();
                    new_process.amount += 1;
                    new_process.memory += process.memory() as f64;

                    continue
                } else {
                    1
                };

                let process = Process {
                    pid: *pid,
                    name: process.name().to_string(),
                    path: process.cwd().unwrap().to_str().unwrap().to_string(),
                    memory: process.memory() as f64,
                    amount
                };

                processes.push(process);
            }
        }
    }
    processes.sort_by_key(|p| p.memory as i64);
    processes.reverse();
    
    processes
}