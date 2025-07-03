#[derive(Default)]
pub struct AppConfig {
    pub auth: AuthConfig,
}

#[derive(Default)]
pub struct AuthConfig {
    pub method: String,
    pub service_account_key_path: Option<String>,
}
