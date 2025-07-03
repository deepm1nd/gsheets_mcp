// End-to-end tests for all transports
use std::sync::Arc;
use gsheets_mcp::google_api_client::GoogleApiClient;
use gsheets_mcp::models::{
    ListSpreadsheetsHandler, ListSpreadsheetsRequest, ListSpreadsheetsResponse,
    CreateSpreadsheetHandler, CreateSpreadsheetRequest, CreateSpreadsheetResponse,
    McpHandler, GetSheetDataHandler, GetSheetDataRequest, SetSheetDataHandler, SetSheetDataRequest,
    BatchGetSheetDataHandler, BatchGetSheetDataRequest, BatchSetSheetDataHandler, BatchSetSheetDataRequest,
    ListDriveFilesHandler, ListDriveFilesRequest, GetDriveFileMetadataHandler, GetDriveFileMetadataRequest
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

#[tokio::test]
async fn test_get_sheet_data_handler_e2e() {
    let client = std::sync::Arc::new(gsheets_mcp::google_api_client::GoogleApiClient::new(std::sync::Arc::new(gsheets_mcp::google_api_client::AuthManager)).await);
    let handler = GetSheetDataHandler { client };
    let req = GetSheetDataRequest {
        spreadsheet_id: "dummy_id".to_string(),
        range: "Sheet1!A1:B2".to_string(),
    };
    let result = handler.handle(req).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.values.len(), 2);
}

#[tokio::test]
async fn test_set_sheet_data_handler_e2e() {
    let client = std::sync::Arc::new(gsheets_mcp::google_api_client::GoogleApiClient::new(std::sync::Arc::new(gsheets_mcp::google_api_client::AuthManager)).await);
    let handler = SetSheetDataHandler { client };
    let req = SetSheetDataRequest {
        spreadsheet_id: "dummy_id".to_string(),
        range: "Sheet1!A1:B2".to_string(),
        values: vec![vec!["A".to_string(), "B".to_string()]],
    };
    let result = handler.handle(req).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.updated_cells, 2);
}

#[tokio::test]
async fn test_batch_get_sheet_data_handler_e2e() {
    let client = Arc::new(GoogleApiClient::new(Arc::new(gsheets_mcp::google_api_client::AuthManager)).await);
    let handler = BatchGetSheetDataHandler { client };
    let req = BatchGetSheetDataRequest {
        spreadsheet_id: "dummy_id".to_string(),
        ranges: vec!["Sheet1!A1:B2".to_string(), "Sheet2!A1:A2".to_string()],
    };
    let result = handler.handle(req).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.values.len(), 2);
}

#[tokio::test]
async fn test_batch_set_sheet_data_handler_e2e() {
    let client = Arc::new(GoogleApiClient::new(Arc::new(gsheets_mcp::google_api_client::AuthManager)).await);
    let handler = BatchSetSheetDataHandler { client };
    let req = BatchSetSheetDataRequest {
        spreadsheet_id: "dummy_id".to_string(),
        updates: vec![
            ("Sheet1!A1:B2".to_string(), vec![vec!["1".to_string(), "2".to_string()]]),
            ("Sheet2!A1:A2".to_string(), vec![vec!["3".to_string()]])
        ],
    };
    let result = handler.handle(req).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.updated_cells, 3);
}

#[tokio::test]
async fn test_list_drive_files_handler_e2e() {
    let client = Arc::new(GoogleApiClient::new(Arc::new(gsheets_mcp::google_api_client::AuthManager)).await);
    let handler = ListDriveFilesHandler { client };
    let result = handler.handle(ListDriveFilesRequest).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.files.len(), 2);
}

#[tokio::test]
async fn test_get_drive_file_metadata_handler_e2e() {
    let client = Arc::new(GoogleApiClient::new(Arc::new(gsheets_mcp::google_api_client::AuthManager)).await);
    let handler = GetDriveFileMetadataHandler { client };
    let req = GetDriveFileMetadataRequest { file_id: "file1_id".to_string() };
    let result = handler.handle(req).await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    assert_eq!(resp.name, "File_file1_id");
    assert_eq!(resp.mime_type, "application/vnd.google-apps.spreadsheet");
    assert_eq!(resp.size, Some(1024));
}
