use std::fs;

pub struct FileHandler {
    pub lines: Vec<String>,
}

impl FileHandler {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn submit(&self) {
        if cfg!(target_os = "windows") {
            self.submit_windows();
        } else {
            self.submit_unix();
        }
    }
    
    fn submit_windows(&self) {
        let path = "C:\\Temp\\ReportBook.txt";
        fs::write(path, self.lines.join("\n").as_bytes()).expect("Failed to create tempfile");

        open::that(path).expect("Failed to open file");
    }
    
    fn submit_unix(&self) {
        let path = "/tmp/ReportBook.txt";
        fs::write(path, self.lines.join("\n").as_bytes()).expect("Failed to create tempfile");

        open::that(path).expect("Failed to open file");
    }
}