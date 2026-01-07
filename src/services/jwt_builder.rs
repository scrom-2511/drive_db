use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
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

    fn get_epoch_time() -> (u64, u64) {
        let current_epoch_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let exp_epoch_time = current_epoch_time + 3600;
        (current_epoch_time, exp_epoch_time)
    }

    fn build(json_data: ServiceAccount) -> Result<String, DriveDbError> {
        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(json_data.private_key_id.clone());
        header.typ = Some("JWT".to_string());

        let (current_epoch_time, exp_epoch_time) = Self::get_epoch_time();

        let claims = Claims {
            iss: json_data.client_email.clone(),
            aud: String::from("https://oauth2.googleapis.com/token"),
            iat: current_epoch_time,
            exp: exp_epoch_time,
            scope: "https://www.googleapis.com/auth/spreadsheets".to_string(),
        };

        let key: EncodingKey = EncodingKey::from_rsa_pem(json_data.private_key.as_bytes()).unwrap();

        let signed_jwt = encode(&header, &claims, &key).unwrap();

        Ok(signed_jwt)
    }
}
