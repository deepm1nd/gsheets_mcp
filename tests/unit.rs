// Unit tests for authentication and API client
use gsheets_mcp::auth_manager::{AuthMethod, AuthManager};
use gsheets_mcp::config::AppConfig;

#[test]
fn test_auth_method_from_str() {
    assert_eq!(AuthMethod::from_str("service_account"), Some(AuthMethod::ServiceAccount));
    assert_eq!(AuthMethod::from_str("oauth2"), Some(AuthMethod::OAuth2));
    assert_eq!(AuthMethod::from_str("adc"), Some(AuthMethod::ApplicationDefault));
    assert_eq!(AuthMethod::from_str("application_default"), Some(AuthMethod::ApplicationDefault));
    assert_eq!(AuthMethod::from_str("unknown"), None);
}

#[tokio::test]
async fn test_auth_manager_new_service_account_missing_key() {
    let config = AppConfig {
        auth: gsheets_mcp::config::AuthConfig {
            method: "service_account".to_string(),
            service_account_key_path: None,
            ..Default::default()
        },
        ..Default::default()
    };
    let result = AuthManager::new(&config).await;
    assert!(result.is_err());
}

// Add more tests for GoogleApiClient as needed, using mocks or stubs.
