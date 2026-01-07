use std::path::{Path, PathBuf};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{errors::DriveDbError, services::jwt_builder::JwtBuilder};

#[derive(Serialize, Deserialize)]
struct AccessToken {
    access_token: String,
    expiry_time: u64,
    file_path: PathBuf,
}

impl AccessToken {
    async fn get_access_token_self(file_path: &Path) -> Result<Self, DriveDbError> {
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

        Ok(Self {
            access_token,
            expiry_time,
            file_path: file_path.to_path_buf(),
        })
    }

    async fn get_access_token(&mut self) -> Result<String, DriveDbError> {
        let (current_epoch_time, _) = JwtBuilder::get_epoch_time();

        if current_epoch_time > self.expiry_time {
            *self = Self::get_access_token_self(&self.file_path).await?;
        };

        Ok(self.access_token.clone())
    }
}
