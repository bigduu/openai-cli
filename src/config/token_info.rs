use std::{
    fs::{self, File},
    io::BufReader,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OpenAIInfo {
    token: String,
    base_url: String,
}

impl OpenAIInfo {
    pub fn new(file_path: String) -> Result<Self> {
        if let Ok(file) = OpenAIInfo::read_token(file_path.clone()) {
            let reader = BufReader::new(file);
            let info: OpenAIInfo =
                serde_yaml::from_reader(reader).expect("Occurred error when reading OpenAIInfo");
            Ok(info)
        } else {
            error!("Occurred error when reading token file: {}", file_path);
            panic!("Occurred error when reading token file: {}", file_path)
        }
    }

    pub fn get_token(&self) -> String {
        format!("Bearer {}", self.token.clone())
    }

    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }

    fn read_token(file_path: String) -> Result<File> {
        let file = match File::open(file_path.clone()) {
            Ok(file) => file,
            Err(e) => {
                let mut default_mut = OpenAIInfo::default();
                let mut token = String::new();
                println!("Please input your token:");
                std::io::stdin()
                    .read_line(&mut token)
                    .expect("Occurred error when reading token");
                token = token.chars().filter(|c| !c.is_whitespace()).collect();
                default_mut.token = token;
                if let Ok(default_file) = serde_yaml::to_string(&default_mut) {
                    fs::write(file_path.clone(), default_file)
                        .expect("Occurred error when writing file");
                    File::open(file_path).expect("Occurred error when opening file")
                } else {
                    error!("Occurred error when serializing OpenAIInfo: {}", e);
                    panic!("Occurred error when serializing OpenAIInfo: {}", e)
                }
            }
        };

        Ok(file)
    }
}

impl Default for OpenAIInfo {
    fn default() -> Self {
        Self {
            token: "".to_string(),
            base_url: "https://api.openai.com".to_string(),
        }
    }
}
