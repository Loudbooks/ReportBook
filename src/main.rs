use sysinfo::System;
use crate::process::prettify_memory;

mod process;
mod app;
mod datagatherers;
mod httphandler;

#[tokio::main]
async fn main() {
    let mut http_handler = httphandler::HttpHandler::new("http://127.0.0.1:25658/upload".to_string());

    let mut sys = System::new();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let total_swap = sys.total_swap();
    let total_memory_used = sys.used_memory();
    let total_swap_used = sys.used_swap();
    let mut cpu = sys.global_cpu_info().vendor_id();
    let cpus = sys.cpus().len();

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

    http_handler.add_line("".to_string());
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
    http_handler.add_line("".to_string());

    let processes = datagatherers::processes::gather_processes(&sys);
    let installed_apps = datagatherers::installed::gather_installed();

    http_handler.add_line(format!("Total Processes: {}", processes.iter().clone().map(|p| p.amount).sum::<u16>()));

    const MEMORY_SPACES: usize = 9;
    const AMOUNT_SPACES: usize = 3;
    let path_spaces: usize = MEMORY_SPACES + AMOUNT_SPACES + 5 + processes.iter().map(|p| p.name.len()).max().unwrap_or(0);

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
    
    http_handler.submit().await;
}
