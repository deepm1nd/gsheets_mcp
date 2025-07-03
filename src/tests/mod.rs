// Unit tests for authentication and API client
#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth_manager::{AuthMethod, AuthManager};
    use crate::config::AppConfig;

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
            auth: crate::config::AuthConfig {
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
}

// End-to-end tests for all transports
#[cfg(test)]
mod e2e {
    use super::*;
    use std::sync::Arc;
    use crate::google_api_client::GoogleApiClient;
    use crate::models::{
        ListSpreadsheetsHandler, ListSpreadsheetsRequest, ListSpreadsheetsResponse,
        CreateSpreadsheetHandler, CreateSpreadsheetRequest, CreateSpreadsheetResponse,
        McpHandler
    };

    #[tokio::test]
    async fn test_list_spreadsheets_handler() {
        let client = Arc::new(GoogleApiClient::new(Arc::new(crate::google_api_client::AuthManager)).await);
        let handler = ListSpreadsheetsHandler { client };
        let result = handler.handle(ListSpreadsheetsRequest).await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(!resp.spreadsheets.is_empty());
    }

    #[tokio::test]
    async fn test_create_spreadsheet_handler() {
        let client = Arc::new(GoogleApiClient::new(Arc::new(crate::google_api_client::AuthManager)).await);
        let handler = CreateSpreadsheetHandler { client };
        let req = CreateSpreadsheetRequest { title: "TestSheet".to_string() };
        let result = handler.handle(req).await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.id.starts_with("created-"));
    }
}

// (Moved all tests to tests/unit.rs and tests/e2e.rs for automatic discovery)
