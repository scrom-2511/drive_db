use reqwest::Client;
use serde_json::Value;

use crate::{errors::DriveDbError, services::access_token::AccessToken};

struct ApiClient {
    client: Client,
    endpoint: String,
    access_token_data: AccessToken,
}

impl ApiClient {
    pub fn new(client: Client, endpoint: String, access_token_data: AccessToken) -> Self {
        Self {
            client,
            endpoint,
            access_token_data,
        }
    }

    async fn post(&mut self, body: Value) -> Result<Value, DriveDbError> {
        let access_token = self.access_token_data.get_access_token().await?;
        let res = self
            .client
            .post(&self.endpoint)
            .bearer_auth(access_token)
            .json(&body)
            .send()
            .await?
            .json::<Value>()
            .await?;
        Ok(res)
    }
}
