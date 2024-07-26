pub struct Process {
    pub name: String,
    pub path: String,
    pub memory: f64,
    pub amount: u16,
}

impl Process {
    pub fn to_string(
        &self,
        memory_spaces: usize,
        amount_spaces: usize,
        path_spaces: usize,
    ) -> String {
        let memory_amount = prettify_memory(self.memory);

        let memory_amount = format!("{:width$}", memory_amount, width = memory_spaces);
        let self_amount = format!("{:width$}", self.amount, width = amount_spaces);

        if self.path == "/" {
            format!("{} │ {} │ {}", self_amount, memory_amount, self.name)
        } else {
            let out = format!("{} | {} | {}", self_amount, memory_amount, self.name);
            let out_len = out.len();
            let spaces = if path_spaces.saturating_sub(out_len) > 0 {
                "─".repeat(path_spaces - out_len)
            } else {
                "".to_string()
            };

            let path = format!("{} {}", spaces, self.path);

            format!(
                "{} │ {} │ {} {}",
                self_amount, memory_amount, self.name, path
            )
        }
    }
}

pub fn prettify_memory(memory: f64) -> String {
    const PRECISION: usize = 1;

    if memory < 1024f64 {
        format!("{:.1$} B", memory, PRECISION)
    } else if memory < 1024f64 * 1024f64 {
        format!("{:.1$} KB", memory / 1024f64, PRECISION)
    } else if memory < 1024f64 * 1024f64 * 1024f64 {
        format!("{:.1$} MB", memory / 1024f64 / 1024f64, PRECISION)
    } else {
        format!("{:.1$} GB", memory / 1024f64 / 1024f64 / 1024f64, PRECISION)
    }
}
