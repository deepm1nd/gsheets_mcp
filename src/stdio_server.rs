// Stdio server implementation for CLI/pipe integration
use std::sync::Arc;
use crate::google_api_client::GoogleApiClient;
use crate::error::McpError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct StdioServer {
    pub api_client: Arc<GoogleApiClient>,
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum StdioCommand {
    ListSpreadsheets,
    CreateSpreadsheet { title: String },
    GetSpreadsheet { spreadsheet_id: String },
    SetSheetValues { spreadsheet_id: String, range: String, values: Vec<Vec<String>> },
    BatchUpdate { spreadsheet_id: String, requests: Vec<serde_json::Value> },
    CopySpreadsheet { spreadsheet_id: String, new_title: String },
    RenameSpreadsheet { spreadsheet_id: String, new_title: String },
    DeleteSpreadsheet { spreadsheet_id: String },
    ExportSpreadsheetPdf { spreadsheet_id: String },
    ListDriveFolders,
    ShareSpreadsheet { spreadsheet_id: String, email: String, role: String },
    GetSpreadsheetPermissions { spreadsheet_id: String },
    GetDriveFileMetadata { file_id: String },
}

#[derive(Serialize)]
pub struct StdioResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl StdioServer {
    pub fn new(api_client: Arc<GoogleApiClient>) -> Self {
        StdioServer { api_client }
    }

    pub async fn run(&self) -> Result<(), McpError> {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();
        let mut stdout = io::stdout();
        while reader.read_line(&mut line).await? > 0 {
            let cmd: Result<StdioCommand, _> = serde_json::from_str(&line);
            let response = match cmd {
                Ok(StdioCommand::ListSpreadsheets) => {
                    match self.api_client.list_spreadsheets().await {
                        Ok(list) => StdioResponse { success: true, data: Some(json!(list)), error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::CreateSpreadsheet { title }) => {
                    match self.api_client.create_spreadsheet(&title).await {
                        Ok(id) => StdioResponse { success: true, data: Some(json!(id)), error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::GetSpreadsheet { spreadsheet_id }) => {
                    match self.api_client.get_spreadsheet(&spreadsheet_id).await {
                        Ok(sheet) => StdioResponse { success: true, data: Some(json!(sheet)), error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::SetSheetValues { spreadsheet_id, range, values }) => {
                    match self.api_client.set_sheet_values(&spreadsheet_id, &range, values).await {
                        Ok(_) => StdioResponse { success: true, data: None, error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::BatchUpdate { spreadsheet_id, requests }) => {
                    let reqs: Vec<google_sheets4::api::Request> = requests.into_iter().filter_map(|v| serde_json::from_value(v).ok()).collect();
                    match self.api_client.batch_update(&spreadsheet_id, reqs).await {
                        Ok(_) => StdioResponse { success: true, data: None, error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::CopySpreadsheet { spreadsheet_id, new_title }) => {
                    match self.api_client.copy_spreadsheet(&spreadsheet_id, &new_title).await {
                        Ok(new_id) => StdioResponse { success: true, data: Some(json!(new_id)), error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::RenameSpreadsheet { spreadsheet_id, new_title }) => {
                    match self.api_client.rename_spreadsheet(&spreadsheet_id, &new_title).await {
                        Ok(_) => StdioResponse { success: true, data: None, error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::DeleteSpreadsheet { spreadsheet_id }) => {
                    match self.api_client.delete_spreadsheet(&spreadsheet_id).await {
                        Ok(_) => StdioResponse { success: true, data: None, error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::ExportSpreadsheetPdf { spreadsheet_id }) => {
                    match self.api_client.export_spreadsheet_pdf(&spreadsheet_id).await {
                        Ok(bytes) => StdioResponse { success: true, data: Some(json!(base64::encode(bytes))), error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::ListDriveFolders) => {
                    match self.api_client.list_drive_folders().await {
                        Ok(folders) => StdioResponse { success: true, data: Some(json!(folders)), error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::ShareSpreadsheet { spreadsheet_id, email, role }) => {
                    match self.api_client.share_spreadsheet(&spreadsheet_id, &email, &role).await {
                        Ok(_) => StdioResponse { success: true, data: None, error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::GetSpreadsheetPermissions { spreadsheet_id }) => {
                    match self.api_client.get_spreadsheet_permissions(&spreadsheet_id).await {
                        Ok(perms) => StdioResponse { success: true, data: Some(json!(perms)), error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(StdioCommand::GetDriveFileMetadata { file_id }) => {
                    match self.api_client.get_drive_file_metadata(&file_id).await {
                        Ok(file) => StdioResponse { success: true, data: Some(json!(file)), error: None },
                        Err(e) => StdioResponse { success: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Err(e) => StdioResponse::<serde_json::Value> { success: false, data: None, error: Some(format!("Parse error: {}", e)) },
            };
            let resp = serde_json::to_string(&response).unwrap_or_else(|_| "{\"success\":false,\"error\":\"Serialization error\"}".to_string());
            stdout.write_all(resp.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            line.clear();
        }
        Ok(())
    }
}
