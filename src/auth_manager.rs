use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum AuthMethod {
    ServiceAccount,
    OAuth2,
    ApplicationDefault,
}

impl AuthMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "service_account" => Some(AuthMethod::ServiceAccount),
            "oauth2" => Some(AuthMethod::OAuth2),
            "adc" | "application_default" => Some(AuthMethod::ApplicationDefault),
            _ => None,
        }
    }
}

pub struct AuthManager;

impl AuthManager {
    pub async fn new(_config: &crate::config::AppConfig) -> Result<Self, String> {
        // Stub: always error if service_account_key_path is None
        if let Some(ref key_path) = _config.auth.service_account_key_path {
            Ok(AuthManager)
        } else {
            Err("Missing service_account_key_path".to_string())
        }
    }
}
