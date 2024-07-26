use std::path::Path;

use serde::Deserialize;
use tokio::io::AsyncWriteExt;
use std::collections::HashMap;
use tokio::fs;
use tokio::io;

use crate::errors::Error;

#[derive(Deserialize, Debug)]
pub struct BodyConfig {
    pub kind: Option<String>,
    pub value: String,
}

#[derive(Deserialize, Debug)]
pub struct FileConfig {
    pub url: String,
    pub method: String,
    pub headers: Option<HashMap<String, String>>,
    pub form: Option<HashMap<String, String>>,
    pub timeout: Option<u64>,
    pub body: Option<BodyConfig>,
}

pub async fn get_file_config(path: &Path) -> Result<FileConfig, Error> {
    let file_contents = match fs::read_to_string(path).await {
        Ok(file_contents) => file_contents,
        Err(e) => {
            let formated_error = format!("Failed to read config file at path {}\n", path.to_string_lossy());
            io::stderr().write(formated_error.as_bytes()).await.unwrap();
            return Err(Error::IOError(e));
        },
    };

    let file_config: FileConfig = match toml::from_str(&file_contents) {
        Ok(file_config) => file_config,
        Err(e) => {
            let formated_error = format!("{} in config file!\n", e.message());
            io::stderr().write(formated_error.as_bytes()).await.unwrap();
            return Err(Error::ConfigParsingError);
        },
    };

    return Ok(file_config);
}
