use reqwest::Client;

pub struct HttpHandler {
    pub url: String,
    pub lines: Vec<String>
}

impl HttpHandler {
    pub fn new(url: String) -> Self {
        Self {
            url,
            lines: Vec::new()
        }
    }

    pub fn add_line(&mut self, line: String) {
        self.lines.push(line);
    }

    pub async fn submit(&self) {
        let client = Client::new();
        let username = whoami::username();
        let title = format!("{}'s Full Report", username);
        
        let res = client.post(&self.url)
            .body(self.lines.join("\n"))
            .header("Content-Type", "text/plain")
            .header("Title", title)
            .send()
            .await;

        if let Err(e) = res {
            eprintln!("Failed to submit data: {}", e);
        }
    }
}