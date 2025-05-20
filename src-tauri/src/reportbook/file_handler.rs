pub struct FileHandler {
    pub lines: Vec<String>,
}

impl FileHandler {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }
}
