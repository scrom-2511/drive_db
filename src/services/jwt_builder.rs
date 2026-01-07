use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::errors::DriveDbError;

#[derive(Debug, Serialize, Deserialize)]
struct ServiceAccount {
    #[serde(rename = "type")]
    type_: String,

    project_id: String,
    private_key_id: String,
    private_key: String,
    client_email: String,
    client_id: String,

    auth_uri: String,
    token_uri: String,
    auth_provider_x509_cert_url: String,
    client_x509_cert_url: String,
    universe_domain: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    iat: u64,
    exp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtBuilder {
    file_data: ServiceAccount,
}

impl JwtBuilder {
    fn new(file_path: &Path) -> Result<Self, DriveDbError> {
        let file = match fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(e) => return Err(DriveDbError::WrongFilePath(e)),
        };

        let file_data = match serde_json::from_str::<ServiceAccount>(&file) {
            Ok(file_data) => file_data,
            Err(e) => return Err(DriveDbError::WrongFile),
        };

        Ok(Self { file_data })
    }
}
