mod auth;
mod command;
mod config_parser;
mod errors;
mod request;

use auth::generate_token_from_config;
use clap::Parser;
use config_parser::{get_auth_config, get_file_config};
use request::{execute_request, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;
use std::error::Error;
use std::{path::Path, str::FromStr, time::Duration};
use tokio::fs;
use tokio::io;
use tokio::io::AsyncWriteExt;

/// File Request maker????
#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Args {
    #[clap(value_parser)]
    file: String,

    #[arg(long)]
    verbose_response: bool,
}

#[tokio::main]
pub async fn main() -> () {
    let args = Args::parse();

    let mut stderr = io::stderr();

    let file_path = Path::new(&args.file);
    let file_config = match get_file_config(file_path).await {
        Ok(file_config) => file_config,
        _ => return,
    };

    let body = match file_config.body {
        Some(body) => match body.kind {
            Some(kind) => match kind.as_str() {
                "file" => match fs::read_to_string(&body.value).await {
                    Ok(body_file_contents) => Some(body_file_contents),
                    Err(_) => {
                        let formated_error =
                            format!("Failed to get body file at path: {}\n", &body.value);
                        stderr.write(formated_error.as_bytes()).await.unwrap();
                        return;
                    }
                },
                "value" => Some(body.value),
                &_ => {
                    stderr.write("Invalid body kind in config\n".as_bytes()).await.unwrap();
                    return;
                }
            },
            None => Some(body.value),
        },
        None => None,
    };

    let timeout = match file_config.timeout {
        Some(timeout) => Some(Duration::from_millis(timeout)),
        None => None,
    };

    let mut options = RequestOptions {
        method: Method::from_str(&file_config.method).unwrap(),
        url: String::from(file_config.url),
        headers: file_config.headers,
        body,
        timeout,
        form: file_config.form,
    };

    match file_config.auth {
        Some(auth_file_path) => {
            let auth_file_path = Path::new(&auth_file_path);
            let auth_file_config = match get_auth_config(&auth_file_path).await {
                Ok(auth_file_config) => auth_file_config,
                _ => return,
            };

            let (header_name, token) = match generate_token_from_config(auth_file_config) {
                Ok(token) => {
                    let header_name = match token.0 {
                        Some(header_name) => header_name,
                        None => "Authorization".to_string(),
                    };

                    (header_name, token.1)
                },
                Err(_) => {
                    stderr.write("Failed to generate token\n".as_bytes()).await.unwrap();
                    return;
                },
            };

            match &mut options.headers {
                Some(headers) => {
                    headers.insert(header_name, token);
                }
                None => {
                    let mut headers = HashMap::new();
                    headers.insert(header_name, token);
                    options.headers = Some(headers);
                }
            };
        }
        None => {}
    };

    let response = match execute_request(options).await {
        Ok(response) => response,
        Err(e) => {
            let err_msg = match e.source() {
                Some(err_msg) => err_msg.to_string(),
                None => "unknown".to_string(),
            };

            let formated_error = format!("Request failed: {}", err_msg);
            stderr.write(formated_error.as_bytes()).await.unwrap();
            return;
        }
    };

    let response_messsage;
    let mut stdout = io::stdout();

    if args.verbose_response {
        let status = response.status();
        let headers: String = response
            .headers()
            .clone()
            .iter()
            .map(|header| format!("{}: {:?}\n", header.0, header.1))
            .collect();
        let body = response.text().await.unwrap();

        response_messsage = format!(
            "\nSTATUS\n---\n{}\n\nHEADERS\n---\n{}\nBODY\n---\n{}",
            status, headers, body
        );
    } else {
        response_messsage = response.text().await.unwrap();
    };

    stdout.write(response_messsage.as_bytes()).await.unwrap();
}
