pub fn gather_hosts() -> Vec<String> {
    let file = if cfg!(target_os = "windows") {
        "C:\\Windows\\System32\\drivers\\etc\\hosts"
    } else {
        "/etc/hosts"
    };

    let hosts = std::fs::read_to_string(file);
    if hosts.is_err() {
        return Vec::new();
    }

    let hosts = hosts.unwrap();
    let hosts = hosts.split('\n');

    hosts
        .filter(|host| !host.starts_with('#'))
        .filter(|host| !host.trim().is_empty())
        .map(|host| host.to_string())
        .collect()
}
