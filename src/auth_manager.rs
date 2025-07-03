use std::fs::File as StdFile;
use std::sync::Arc;
use google_sheets4::hyper::client::HttpConnector;
use google_sheets4::hyper_rustls::HttpsConnectorBuilder;
use serde_json;
use tokio::task;
use yup_oauth2::{authenticator::Authenticator, ServiceAccountAuthenticator, ServiceAccountKey};

use crate::config::AppConfig;
use crate::error::McpError;
use crate::logging::log_info;

#[derive(Debug, Clone)]
pub enum AuthMethod {
    ServiceAccount,
    OAuth2,
    ApplicationDefault,
}

impl AuthMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "service_account" => Some(AuthMethod::ServiceAccount),
            "oauth2" => Some(AuthMethod::OAuth2),
            "adc" | "application_default" => Some(AuthMethod::ApplicationDefault),
            _ => None,
        }
    }
}

pub struct AuthManager {
    pub method: AuthMethod,
    pub authenticator: Authenticator<google_sheets4::hyper_rustls::HttpsConnector<google_sheets4::hyper::client::HttpConnector>>,
    pub client: google_sheets4::hyper::Client<google_sheets4::hyper_rustls::HttpsConnector<google_sheets4::hyper::client::HttpConnector>, google_sheets4::hyper::Body>,
}

impl AuthManager {
    pub async fn new(config: &AppConfig) -> Result<Self, McpError> {
        let method = AuthMethod::from_str(&config.auth.method)
            .ok_or_else(|| McpError::Other("Invalid auth method".to_string()))?;
        log_info(&format!("Initializing AuthManager with method: {:?}", method));
        match method {
            AuthMethod::ServiceAccount => {
                let key_path = config.auth.service_account_key_path.as_ref().ok_or_else(|| McpError::Other("service_account_key_path must be set for Service Account auth".to_string()))?;
                let key_path = key_path.clone();
                let key = task::spawn_blocking(move || {
                    let file = StdFile::open(&key_path).map_err(|e| McpError::Auth(format!("Failed to open service account key: {}", e)))?;
                    serde_json::from_reader(file).map_err(|e| McpError::Auth(format!("Failed to parse service account key: {}", e)))
                }).await.map_err(|e| McpError::Auth(format!("Failed to spawn blocking task: {}", e)))??;
                let https = google_sheets4::hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .build();
                let client: google_sheets4::hyper::Client<google_sheets4::hyper_rustls::HttpsConnector<google_sheets4::hyper::client::HttpConnector>, google_sheets4::hyper::Body> = google_sheets4::hyper::Client::builder().build(https);
                let auth = ServiceAccountAuthenticator::builder(key).build().await.map_err(|e| McpError::Auth(format!("Failed to build authenticator: {}", e)))?;
                Ok(AuthManager { method, authenticator: auth, client })
            }
            _ => Err(McpError::Other("Only service_account auth is implemented".to_string())),
        }
    }

    pub fn get_authenticator(&self) -> &Authenticator<google_sheets4::hyper_rustls::HttpsConnector<google_sheets4::hyper::client::HttpConnector>> {
        &self.authenticator
    }
    pub async fn get_token(&self, scopes: &[&str]) -> Result<String, McpError> {
        let token = self.authenticator.token(scopes).await.map_err(|e| McpError::Auth(format!("Failed to get token: {}", e)))?;
        Ok(token.token().expect("token missing").to_string())
    }
}

// Unit tests and further integration will be added in the next steps.
