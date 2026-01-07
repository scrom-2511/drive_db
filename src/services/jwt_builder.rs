use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{Algorithm, Header};
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

