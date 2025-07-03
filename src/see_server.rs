// SEE (Streaming Event Exchange) server implementation
use std::sync::Arc;
use crate::google_api_client::GoogleApiClient;
use crate::error::McpError;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

pub struct SeeServer {
    pub api_client: Arc<GoogleApiClient>,
}

#[derive(Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum SeeEvent {
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
pub struct SeeResponse<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl SeeServer {
    pub fn new(api_client: Arc<GoogleApiClient>) -> Self {
        SeeServer { api_client }
    }

    pub async fn run(&self) -> Result<(), McpError> {
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);
        let mut line = String::new();
        let mut stdout = io::stdout();
        while reader.read_line(&mut line).await? > 0 {
            let evt: Result<SeeEvent, _> = serde_json::from_str(&line);
            let response = match evt {
                Ok(SeeEvent::ListSpreadsheets) => {
                    match self.api_client.list_spreadsheets().await {
                        Ok(list) => SeeResponse { ok: true, data: Some(json!(list)), error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::CreateSpreadsheet { title }) => {
                    match self.api_client.create_spreadsheet(&title).await {
                        Ok(id) => SeeResponse { ok: true, data: Some(json!(id)), error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::GetSpreadsheet { spreadsheet_id }) => {
                    match self.api_client.get_spreadsheet(&spreadsheet_id).await {
                        Ok(sheet) => SeeResponse { ok: true, data: Some(json!(sheet)), error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::SetSheetValues { spreadsheet_id, range, values }) => {
                    match self.api_client.set_sheet_values(&spreadsheet_id, &range, values).await {
                        Ok(_) => SeeResponse { ok: true, data: None, error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::BatchUpdate { spreadsheet_id, requests }) => {
                    let reqs: Vec<google_sheets4::api::Request> = requests.into_iter().filter_map(|v| serde_json::from_value(v).ok()).collect();
                    match self.api_client.batch_update(&spreadsheet_id, reqs).await {
                        Ok(_) => SeeResponse { ok: true, data: None, error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::CopySpreadsheet { spreadsheet_id, new_title }) => {
                    match self.api_client.copy_spreadsheet(&spreadsheet_id, &new_title).await {
                        Ok(new_id) => SeeResponse { ok: true, data: Some(json!(new_id)), error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::RenameSpreadsheet { spreadsheet_id, new_title }) => {
                    match self.api_client.rename_spreadsheet(&spreadsheet_id, &new_title).await {
                        Ok(_) => SeeResponse { ok: true, data: None, error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::DeleteSpreadsheet { spreadsheet_id }) => {
                    match self.api_client.delete_spreadsheet(&spreadsheet_id).await {
                        Ok(_) => SeeResponse { ok: true, data: None, error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::ExportSpreadsheetPdf { spreadsheet_id }) => {
                    match self.api_client.export_spreadsheet_pdf(&spreadsheet_id).await {
                        Ok(bytes) => SeeResponse { ok: true, data: Some(json!(base64::encode(bytes))), error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::ListDriveFolders) => {
                    match self.api_client.list_drive_folders().await {
                        Ok(folders) => SeeResponse { ok: true, data: Some(json!(folders)), error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::ShareSpreadsheet { spreadsheet_id, email, role }) => {
                    match self.api_client.share_spreadsheet(&spreadsheet_id, &email, &role).await {
                        Ok(_) => SeeResponse { ok: true, data: None, error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::GetSpreadsheetPermissions { spreadsheet_id }) => {
                    match self.api_client.get_spreadsheet_permissions(&spreadsheet_id).await {
                        Ok(perms) => SeeResponse { ok: true, data: Some(json!(perms)), error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Ok(SeeEvent::GetDriveFileMetadata { file_id }) => {
                    match self.api_client.get_drive_file_metadata(&file_id).await {
                        Ok(file) => SeeResponse { ok: true, data: Some(json!(file)), error: None },
                        Err(e) => SeeResponse { ok: false, data: None, error: Some(e.to_string()) },
                    }
                }
                Err(e) => SeeResponse::<serde_json::Value> { ok: false, data: None, error: Some(format!("Parse error: {}", e)) },
            };
            let resp = serde_json::to_string(&response).unwrap_or_else(|_| "{\"ok\":false,\"error\":\"Serialization error\"}".to_string());
            stdout.write_all(resp.as_bytes()).await?;
            stdout.write_all(b"\n").await?;
            line.clear();
        }
        Ok(())
    }
}
