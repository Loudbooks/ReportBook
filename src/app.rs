pub struct App {
    pub name: String,
    pub version: String,
    pub author: String,
}

impl App {
    pub fn to_string(&self, name_spaces: usize, version_spaces: usize, author_spaces: usize) -> String {
        let name = format!("{:width$}", self.name, width = name_spaces);
        let version = format!("{:width$}", self.version, width = version_spaces);
        let author = format!("{:width$}", self.author, width = author_spaces);

        format!("{} │ v{} │ {}", name, version, author)
    }
}