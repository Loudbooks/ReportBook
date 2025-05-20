use crate::reportbook::process::prettify_memory;
use crate::reportbook::{datagatherers, file_handler};
use machine_info::Machine;
use sysinfo::System;

pub fn collect_log() -> String {
    let mut file_handler = file_handler::FileHandler::new();

    let mut sys = System::new_all();
    sys.refresh_all();

    let mut machine = Machine::new();

    let total_memory = sys.total_memory();
    let total_swap = sys.total_swap();
    let total_memory_used = sys.used_memory();
    let total_swap_used = sys.used_swap();
    let cpu = sys.cpus().first().unwrap().brand();
    let cpus = sys.cpus().len();
    let gpus = machine.system_info().graphics;
    let username = whoami::username();
    let is_alphanumeric_username = username.chars().all(char::is_alphanumeric);
    let log_creation = chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string();

    let log_creation_str = {
        let id = "Log Creation: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), log_creation)
    };

    let os_name = format!(
        "{}, {}",
        System::name().unwrap_or("Unknown".to_string()),
        System::os_version().unwrap_or("Unknown".to_string())
    );

    const INFORMATION_SPACES: usize = 20;

    let os_name_str = {
        let id = "Operating System: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), os_name)
    };

    let total_memory_str = {
        let id = "Total Memory: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!(
            "{}{}{}",
            id,
            " ".repeat(spaces),
            prettify_memory(total_memory as f64)
        )
    };

    let total_swap_str = {
        let id = "Total Swap: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!(
            "{}{}{}",
            id,
            " ".repeat(spaces),
            prettify_memory(total_swap as f64)
        )
    };

    let total_memory_used_str = {
        let id = "Total Memory Used: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!(
            "{}{}{}",
            id,
            " ".repeat(spaces),
            prettify_memory(total_memory_used as f64)
        )
    };

    let total_swap_used_str = {
        let id = "Total Swap Used: ";

        let spaces = INFORMATION_SPACES - id.len();
        format!(
            "{}{}{}",
            id,
            " ".repeat(spaces),
            prettify_memory(total_swap_used as f64)
        )
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

        let mut index = 0;
        for graphics in &gpus {
            let str = graphics.name.to_string();

            gpu_str.push_str(
                format!("{}{}{}", id, " ".repeat(spaces).as_str(), str.as_str()).as_str(),
            );

            if index < &gpus.len() - 1 {
                gpu_str.push_str("\n");
            }

            index += 1;
        }

        gpu_str
    };

    let bad_chars = username
        .chars()
        .filter(|c| !c.is_alphanumeric())
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    let non_alphanumeric = format!("Contains {}", bad_chars.clone());

    let alphanumeric_username = {
        let id = "Username: ";

        let value = if is_alphanumeric_username {
            "Alphanumeric"
        } else {
            non_alphanumeric.as_str()
        };

        let spaces = INFORMATION_SPACES - id.len();
        format!("{}{}{}", id, " ".repeat(spaces), value)
    };

    file_handler.add_line(log_creation_str);

    file_handler.add_line("".to_string());

    file_handler.add_line(os_name_str.to_string());
    file_handler.add_line(total_memory_str);
    if total_swap > 0 {
        file_handler.add_line(total_swap_str);
    }
    file_handler.add_line(total_memory_used_str);
    if total_swap_used > 0 {
        file_handler.add_line(total_swap_used_str);
    }
    file_handler.add_line(cpu);
    file_handler.add_line(cpus);
    if !gpu.is_empty() {
        file_handler.add_line(gpu);
    }
    file_handler.add_line(alphanumeric_username);

    file_handler.add_line("".to_string());

    let processes = datagatherers::processes::gather_processes(&sys);
    let installed_apps = datagatherers::installed::gather_installed();

    file_handler.add_line(format!(
        "Total Processes: {}",
        processes.iter().clone().map(|p| p.amount).sum::<u16>()
    ));

    const MEMORY_SPACES: usize = 9;
    const AMOUNT_SPACES: usize = 3;
    let path_spaces: usize = MEMORY_SPACES
        + AMOUNT_SPACES
        + 7
        + processes.iter().map(|p| p.name.len()).max().unwrap_or(0);

    for process in processes {
        file_handler.add_line(process.to_string(MEMORY_SPACES, AMOUNT_SPACES, path_spaces));
    }

    file_handler.add_line("".to_string());
    file_handler.add_line("Installed Apps:".to_string());

    let name_spaces = installed_apps
        .iter()
        .map(|a| a.name.len())
        .max()
        .unwrap_or(0);
    let version_spaces = installed_apps
        .iter()
        .map(|a| a.version.len())
        .max()
        .unwrap_or(0);
    let author_spaces = installed_apps
        .iter()
        .map(|a| a.author.len())
        .max()
        .unwrap_or(0);

    for app in installed_apps {
        file_handler.add_line(app.to_string(name_spaces, version_spaces, author_spaces));
    }

    let hosts = datagatherers::hosts::gather_hosts();

    if !hosts.is_empty() {
        file_handler.add_line("".to_string());
        file_handler.add_line("Hosts:".to_string());

        for host in hosts {
            file_handler.add_line(host);
        }
    }

    file_handler.lines.join("\n")
}
