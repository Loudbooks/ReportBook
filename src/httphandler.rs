use std::io::Cursor;

pub struct HttpHandler {
    pub url: String,
    pub lines: Vec<String>,
}

impl HttpHandler {
    pub fn new(url: String) -> Self {
        Self {
            url,
            lines: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub fn submit(&self, name: &str) {
        println!("Uploading your paste...");
        let title = format!("{}'s ReportBook", name);

        let result = ureq::post(&self.url)
            .set("content-type", "text/plain")
            .set("title", &title)
            .send(Cursor::new(self.lines.join("\n")))
            .unwrap()
            .into_string()
            .unwrap();

        println!("View your report at: {}", result);
        println!("Share this link with whoever asked you to run this report!")
    }
}
