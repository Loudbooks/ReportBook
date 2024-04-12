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
        let title = format!("{}'s FullReport", username);

        let result = client.post(&self.url)
            .body(self.lines.join("\n"))
            .header("content-type", "text/plain")
            .header("title", title)
            .send()
            .await.unwrap();
        
        println!("View your report at: {}", result.text().await.unwrap());
    }
}