use std::path::Path;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{errors::DriveDbError, services::jwt_builder::JwtBuilder};

#[derive(Serialize, Deserialize)]
struct AccessToken {
    access_token: String,
    expiry_time: u64,
}

impl AccessToken {
    async fn get_access_token(file_path: &Path) -> Result<(String, Self), DriveDbError> {
        let (signed_jwt, expiry_time) = JwtBuilder::new(file_path)?;
        let client = Client::new();
        let token_resp: Value = client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &signed_jwt),
            ])
            .send()
            .await?
            .json()
            .await?;

        let access_token = token_resp["access_token"].to_string();

        Ok((
            access_token.clone(),
            Self {
                access_token,
                expiry_time,
            },
        ))
    }
}
