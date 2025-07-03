// End-to-end tests for all transports
use std::sync::Arc;
use gsheets_mcp::google_api_client::GoogleApiClient;
use gsheets_mcp::models::{
    ListSpreadsheetsHandler, ListSpreadsheetsRequest, ListSpreadsheetsResponse,
    CreateSpreadsheetHandler, CreateSpreadsheetRequest, CreateSpreadsheetResponse,
    McpHandler
};

#[tokio::test]
async fn test_list_spreadsheets_handler() {
    let client = Arc::new(GoogleApiClient::new(Arc::new(gsheets_mcp::google_api_client::AuthManager)).await);
    let handler = ListSpreadsheetsHandler { client };
    let result = handler.handle(ListSpreadsheetsRequest).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(!resp.spreadsheets.is_empty());
}

#[tokio::test]
async fn test_create_spreadsheet_handler() {
    let client = Arc::new(GoogleApiClient::new(Arc::new(gsheets_mcp::google_api_client::AuthManager)).await);
    let handler = CreateSpreadsheetHandler { client };
    let req = CreateSpreadsheetRequest { title: "TestSheet".to_string() };
    let result = handler.handle(req).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert!(resp.id.starts_with("created-"));
}
