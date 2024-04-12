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

    pub async fn submit(&self, name: &str) {
        println!("Uploading your paste...");
        let client = Client::new();
        let title = format!("{}'s FullReport", name);

        let result = client.post(&self.url)
            .body(self.lines.join("\n"))
            .header("content-type", "text/plain")
            .header("title", title)
            .send()
            .await.unwrap();
        
        println!("View your report at: {}", result.text().await.unwrap());
        println!("Share this link with whoever asked you to run this report!")
    }
}