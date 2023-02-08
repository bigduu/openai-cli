use std::{fs::File, io::BufReader};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIInfo {
    token: String,
    base_url: String,
}

impl OpenAIInfo {
    pub fn new(file_path: String) -> Result<Self> {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        let info: OpenAIInfo =
            serde_yaml::from_reader(reader).expect("Occurred error when reading OpenAIInfo");
        Ok(info)
    }

    pub fn get_token(&self) -> String {
        format!("Bearer {}", self.token.clone())
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }
}
