use base64::Engine;

use crate::config_parser::AuthConfig;
use crate::errors::Error;

pub fn generate_token_from_config(config: AuthConfig) -> Result<(Option<String>, String), Error> {
    if config.resource.as_str() != "auth" {
        return Err(Error::AuthError);
    }

    let token = match config.kind.as_str() {
        "basic" => {
            let basic_config = match config.basic {
                Some(basic_config) => basic_config,
                None => return Err(Error::AuthError)
            };

            format!("Basic {}", base64::prelude::BASE64_STANDARD.encode(format!("{}:{}", basic_config.username, basic_config.password)))
        },
        "bearer" => {
            let bearer_token = match config.bearer {
                Some(bearer_token) => bearer_token,
                None => return Err(Error::AuthError)
            };

            format!("Bearer {}", bearer_token)
        },
        &_ => {
            return Err(Error::AuthError);
        }
    };

    Ok((config.header_name, token))
}
