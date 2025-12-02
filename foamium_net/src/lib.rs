use anyhow::Result;
use reqwest::blocking::Client;

pub struct NetworkManager {
    client: Client,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn fetch_text(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send()?;
        let text = response.text()?;
        Ok(text)
    }
}
