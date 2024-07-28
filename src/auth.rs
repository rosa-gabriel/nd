use std::collections::HashMap;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use base64::Engine;
use serde::Deserialize;

use crate::config_parser::AuthConfig;
use crate::errors::Error;

#[derive(Deserialize)]
struct OAuthTokenResponse {
    access_token: String
}

pub async fn generate_token_from_config(
    config: AuthConfig,
) -> Result<(Option<String>, String), Error> {
    if config.resource.as_str() != "auth" {
        return Err(Error::AuthError);
    }

    let token = match config.kind.as_str() {
        "basic" => {
            let basic_config = match config.basic {
                Some(basic_config) => basic_config,
                None => return Err(Error::AuthError),
            };

            format!(
                "Basic {}",
                base64::prelude::BASE64_STANDARD.encode(format!(
                    "{}:{}",
                    basic_config.username, basic_config.password
                ))
            )
        }
        "bearer" => {
            let bearer_token = match config.bearer {
                Some(bearer_token) => bearer_token,
                None => return Err(Error::AuthError),
            };

            format!("Bearer {}", bearer_token)
        }
        "oauth2" => {
            let oauth2_config = match config.oauth2 {
                Some(oauth2_config) => oauth2_config,
                None => return Err(Error::AuthError),
            };

            let mut url_encode_form: HashMap<String, String> = HashMap::new();

            url_encode_form.insert("client_id".to_string(), oauth2_config.client_id.clone());
            match &oauth2_config.client_secret {
                Some(secret) => {
                    url_encode_form.insert("client_secret".to_string(), secret.clone());
                }
                None => todo!(),
            };
            url_encode_form.insert("grant_type".to_string(), "authorization_code".to_owned());
            url_encode_form.insert("redirect_uri".to_string(), "http://localhost:12345/callback".to_owned());

            webbrowser::open(format!("{}/protocol/openid-connect/auth?response_type=code&client_id=test&redirect_uri=http%3A%2F%2Flocalhost%3A12345%2Fcallback&scope=openid&state=something", oauth2_config.issuer_url).as_str()).unwrap();

            /* TODO: Get reponse code (probably need to run a http server that will be the redirect uri);
             * What postman does probably is that it runs a chromium like instance that it itself
             * maanges and when it sees that the browser has redirected to a uri that matches with
             * the given uri it extracts the `code` query string from the current url */
            let mut reader = BufReader::new(io::stdin());
            let mut buffer = String::new();
            _ = reader.read_line(&mut buffer);

            println!("{}", buffer);

            url_encode_form.insert(
                "code".to_string(),
                "2eb74992-b57b-405f-9bce-f5c4a689a0c3.94f91019-f275-4caf-be1f-53883cbffc22.cd28f294-c1df-4f75-afbd-7d9c83052f58".to_owned(),
            );

            let token_response: OAuthTokenResponse = reqwest::Client::new()
                .post(format!("{}/protocol/openid-connect/token", oauth2_config.issuer_url))
                .form(&url_encode_form)
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            println!("{}", token_response.access_token);
            format!("Bearer {}", "dskfjdsl")
        }
        &_ => {
            return Err(Error::AuthError);
        }
    };

    println!("{}", token);

    Ok((config.header_name, token))
}
