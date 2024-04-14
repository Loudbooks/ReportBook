use std::io::{stdin, stdout, Write};
use machine_info::Machine;
use sysinfo::System;
use crate::process::prettify_memory;

mod process;
mod app;
mod datagatherers;
mod httphandler;

#[tokio::main]
async fn main() {
    let mut http_handler = httphandler::HttpHandler::new("https://pastebook.dev/upload".to_string());

    let mut sys = System::new_all();
    sys.refresh_all();

    let mut machine = Machine::new();

    let total_memory = sys.total_memory();
    let total_swap = sys.total_swap();
    let total_memory_used = sys.used_memory();
    let total_swap_used = sys.used_swap();
    let mut cpu = sys.cpus().get(0).unwrap().brand();
    let cpus = sys.cpus().len();
    let gpus = machine.system_info().graphics;
    let is_alphanumeric_username = whoami::username().chars().all(char::is_alphanumeric);

    if cfg!(target_arch = "aarch64") && cfg!(target_os = "macos") {
        cpu = "Apple Silicon";
    }

    let os_name = format!("{}, {}", System::name().unwrap_or("Unknown".to_string()), System::os_version().unwrap_or("Unknown".to_string()));

    const INFORMATION_SPACES: usize = 20;

    let os_name_str = {
        let id = "Operating System: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), os_name)
    };

    let total_memory_str = {
        let id = "Total Memory: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), prettify_memory(total_memory as f64))
    };

    let total_swap_str = {
        let id = "Total Swap: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), prettify_memory(total_swap as f64))
    };

    let total_memory_used_str = {
        let id = "Total Memory Used: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), prettify_memory(total_memory_used as f64))
    };

    let total_swap_used_str = {
        let id = "Total Swap Used: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), prettify_memory(total_swap_used as f64))
    };

    let cpu = {
        let id = "CPU: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), cpu)
    };

    let cpus = {
        let id = "CPUs: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), cpus)
    };

    let gpu = {
        let id = "GPU: ";
        let mut gpu_str = String::new();
        let spaces = INFORMATION_SPACES - id.len();

        for graphics in gpus {
            let str = format!("{}", graphics.name);

            gpu_str.push_str(format!("{}{}{}\n", id, " ".repeat(spaces).as_str(), str.as_str()).as_str())
        }

        gpu_str
    };

    let alphanumeric_username = {
        let id = "Username: ";

        let value = if is_alphanumeric_username {
            "Alphanumeric"
        } else {
            "None-Alphanumeric"
        };

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), value)
    };

    http_handler.add_line(os_name_str.to_string());
    http_handler.add_line(total_memory_str);
    if total_swap > 0 {
        http_handler.add_line(total_swap_str);
    }
    http_handler.add_line(total_memory_used_str);
    if total_swap_used > 0 {
        http_handler.add_line(total_swap_used_str);
    }
    http_handler.add_line(cpu);
    http_handler.add_line(cpus);
    http_handler.add_line(gpu);
    http_handler.add_line(alphanumeric_username);
    http_handler.add_line("".to_string());

    let processes = datagatherers::processes::gather_processes(&sys);
    let installed_apps = datagatherers::installed::gather_installed();

    http_handler.add_line(format!("Total Processes: {}", processes.iter().clone().map(|p| p.amount).sum::<u16>()));

    const MEMORY_SPACES: usize = 9;
    const AMOUNT_SPACES: usize = 3;
    let path_spaces: usize = MEMORY_SPACES + AMOUNT_SPACES + 7 + processes.iter().map(|p| p.name.len()).max().unwrap_or(0);

    for process in processes {
        http_handler.add_line(process.to_string(MEMORY_SPACES, AMOUNT_SPACES, path_spaces));
    }

    http_handler.add_line("".to_string());
    http_handler.add_line("Installed Apps:".to_string());

    let name_spaces = installed_apps.iter().map(|a| a.name.len()).max().unwrap_or(0);
    let version_spaces = installed_apps.iter().map(|a| a.version.len()).max().unwrap_or(0);
    let author_spaces = installed_apps.iter().map(|a| a.author.len()).max().unwrap_or(0);

    for app in installed_apps {
        http_handler.add_line(app.to_string(name_spaces, version_spaces, author_spaces));
    }

    let hosts = datagatherers::hosts::gather_hosts();

    if !hosts.is_empty() {
        http_handler.add_line("".to_string());
        http_handler.add_line("Hosts:".to_string());

        for host in hosts {
            http_handler.add_line(host);
        }
    }

    println!("Enter your username: ");
    let username = user_input();
    
    http_handler.submit(username.as_str()).await;
    
    wait_for_enter("exit")
}

fn user_input() -> String {
    let mut input= String::new();

    stdout().flush().expect("Failed to flush");
    stdin().read_line(&mut input).expect("Did not enter a correct string");

    input = input.trim().to_string();

    input
}

fn wait_for_enter(message: &str) {
    println!("Press enter to {}.", message);
    let _ = stdin().read_line(&mut String::new());
}