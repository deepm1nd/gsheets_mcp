// Environment/config file parsing and validation

use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct AuthConfig {
    pub method: String, // "service_account", "oauth2", or "adc"
    pub service_account_key_path: Option<String>,
    pub oauth2_client_secret_path: Option<String>,
    // Add more fields as needed for extensibility
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub auth: AuthConfig,
    pub google_drive_folder_id: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

impl AppConfig {
    pub fn validate(&self) -> Result<(), String> {
        match self.auth.method.to_lowercase().as_str() {
            "service_account" => {
                if self.auth.service_account_key_path.is_none() {
                    return Err("service_account_key_path must be set for Service Account auth".to_string());
                }
            }
            "oauth2" => {
                if self.auth.oauth2_client_secret_path.is_none() {
                    return Err("oauth2_client_secret_path must be set for OAuth2 auth".to_string());
                }
            }
            "adc" | "application_default" => {
                // No required fields for ADC
            }
            _ => return Err("Invalid auth method specified".to_string()),
        }
        Ok(())
    }
}

pub fn load_config() -> Result<AppConfig, ConfigError> {
    dotenv::dotenv().ok();
    let mut s = Config::default();
    s.merge(File::with_name("config").required(false))?;
    s.merge(config::Environment::default())?;
    let config: AppConfig = s.try_deserialize()?;
    config.validate().map_err(ConfigError::Message)?;
    Ok(config)
}
