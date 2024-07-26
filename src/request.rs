use std::{collections::HashMap, time::Duration};
use reqwest::Method;

pub struct RequestOptions {
    pub method: Method,
    pub url: String,
    pub headers: Option<HashMap<String, String>>,
    pub form: Option<HashMap<String, String>>,
    pub body: Option<String>,
    pub timeout: Option<Duration>,
}

pub async fn execute_request(options: RequestOptions) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let mut request_builder = client.request(options.method, options.url);

    match options.headers {
        Some(headers) => {
            for header in headers.iter() {
                request_builder = request_builder.header(header.0, header.1);
            }
        }
        None => {}
    }

    match options.body {
        Some(body) => request_builder = request_builder.body(body),
        None => {}
    }

    match options.timeout {
        Some(timeout) => request_builder = request_builder.timeout(timeout),
        None => {}
    }

    match options.form {
        Some(form) => {
            request_builder = request_builder.form(&form);
        }
        None => {}
    }

    request_builder.send().await
}

