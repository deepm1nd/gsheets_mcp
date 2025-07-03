// Abstraction over Google Sheets/Drive APIs
use std::sync::Arc;
use google_sheets4::Sheets;
use google_drive3::DriveHub;
use google_sheets4::hyper::{Client, client::HttpConnector, Body};
use google_sheets4::hyper_rustls::HttpsConnector;
use yup_oauth2::{ServiceAccountAuthenticator, ServiceAccountKey};
use crate::auth_manager::AuthManager;
use crate::error::McpError;
use crate::logging::{log_info, log_error};
use google_sheets4::client::GetToken;

pub struct GoogleApiClient {
    sheets: Sheets<google_sheets4::hyper_rustls::HttpsConnector<google_sheets4::hyper::client::HttpConnector>>,
    drive: DriveHub<google_sheets4::hyper_rustls::HttpsConnector<google_sheets4::hyper::client::HttpConnector>>,
    auth_manager: Arc<AuthManager>,
}

impl GoogleApiClient {
    pub async fn new(
        auth_manager: Arc<AuthManager>,
    ) -> Result<Self, McpError> {
        let client: google_sheets4::hyper::Client<google_sheets4::hyper_rustls::HttpsConnector<google_sheets4::hyper::client::HttpConnector>, google_sheets4::hyper::Body> = auth_manager.client.clone();
        let real_auth = RealTokenProvider::new(auth_manager.clone());
        let sheets = Sheets::new(client.clone(), real_auth.clone());
        let drive = DriveHub::new(client, real_auth);
        Ok(GoogleApiClient { sheets, drive, auth_manager })
    }

    pub async fn list_spreadsheets(&self) -> Result<Vec<String>, McpError> {
        log_info("Listing spreadsheets via Google Drive API");
        let scopes = &["https://www.googleapis.com/auth/drive.readonly"];
        let token = self.auth_manager.get_token(scopes).await?;
        let result = self.drive.files().list()
            .q("mimeType='application/vnd.google-apps.spreadsheet'")
            .doit().await;
        match result {
            Ok((_, file_list)) => {
                let names = file_list.files
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|f| f.name)
                    .collect();
                Ok(names)
            }
            Err(e) => {
                log_error(&format!("Failed to list spreadsheets: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    pub async fn create_spreadsheet(&self, title: &str) -> Result<String, McpError> {
        log_info(&format!("Creating spreadsheet: {}", title));
        let scopes = &["https://www.googleapis.com/auth/spreadsheets"];
        let token = self.auth_manager.get_token(scopes).await?;
        let mut spreadsheet = google_sheets4::api::Spreadsheet::default();
        spreadsheet.properties = Some(google_sheets4::api::SpreadsheetProperties {
            title: Some(title.to_string()),
            ..Default::default()
        });
        let result = self.sheets.spreadsheets().create(spreadsheet).doit().await;
        match result {
            Ok((_, created)) => {
                let id = created.spreadsheet_id.unwrap_or_default();
                Ok(id)
            }
            Err(e) => {
                log_error(&format!("Failed to create spreadsheet: {}", e));
                Err(McpError::ApiError(format!("Sheets API error: {}", e)))
            }
        }
    }

    pub async fn get_spreadsheet(&self, spreadsheet_id: &str) -> Result<google_sheets4::api::Spreadsheet, McpError> {
        log_info(&format!("Getting spreadsheet: {}", spreadsheet_id));
        let scopes = &["https://www.googleapis.com/auth/spreadsheets.readonly"];
        let token = self.auth_manager.get_token(scopes).await?;
        let result = self.sheets.spreadsheets().get(spreadsheet_id).doit().await;
        match result {
            Ok((_, sheet)) => Ok(sheet),
            Err(e) => {
                log_error(&format!("Failed to get spreadsheet: {}", e));
                Err(McpError::ApiError(format!("Sheets API error: {}", e)))
            }
        }
    }

    pub async fn set_sheet_values(&self, spreadsheet_id: &str, range: &str, values: Vec<Vec<String>>) -> Result<(), McpError> {
        log_info(&format!("Setting values in spreadsheet {} range {}", spreadsheet_id, range));
        let scopes = &["https://www.googleapis.com/auth/spreadsheets"];
        let token = self.auth_manager.get_token(scopes).await?;
        let value_range = google_sheets4::api::ValueRange {
            range: Some(range.to_string()),
            values: Some(values.into_iter().map(|row| row.into_iter().map(serde_json::Value::String).collect()).collect()),
            ..Default::default()
        };
        let result = self.sheets.spreadsheets().values_update(value_range, spreadsheet_id, range)
            .value_input_option("RAW")
            .doit().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log_error(&format!("Failed to set sheet values: {}", e));
                Err(McpError::ApiError(format!("Sheets API error: {}", e)))
            }
        }
    }

    pub async fn batch_update(&self, spreadsheet_id: &str, requests: Vec<google_sheets4::api::Request>) -> Result<(), McpError> {
        log_info(&format!("Batch updating spreadsheet: {}", spreadsheet_id));
        let scopes = &["https://www.googleapis.com/auth/spreadsheets"];
        let token = self.auth_manager.get_token(scopes).await?;
        let req = google_sheets4::api::BatchUpdateSpreadsheetRequest {
            requests: Some(requests),
            ..Default::default()
        };
        let result = self.sheets.spreadsheets().batch_update(req, spreadsheet_id).doit().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log_error(&format!("Failed batch update: {}", e));
                Err(McpError::ApiError(format!("Sheets API error: {}", e)))
            }
        }
    }

    pub async fn share_spreadsheet(&self, spreadsheet_id: &str, email: &str, role: &str) -> Result<(), McpError> {
        log_info(&format!("Sharing spreadsheet {} with {} as {}", spreadsheet_id, email, role));
        let scopes = &["https://www.googleapis.com/auth/drive"];
        let token = self.auth_manager.get_token(scopes).await?;
        let mut permission = google_drive3::api::Permission::default();
        permission.type_ = Some("user".to_string());
        permission.role = Some(role.to_string());
        permission.email_address = Some(email.to_string());
        let result = self.drive.permissions().create(permission, spreadsheet_id)
            .send_notification_email(false)
            .doit().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log_error(&format!("Failed to share spreadsheet: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    pub async fn get_spreadsheet_permissions(&self, spreadsheet_id: &str) -> Result<Vec<google_drive3::api::Permission>, McpError> {
        log_info(&format!("Getting permissions for spreadsheet {}", spreadsheet_id));
        let scopes = &["https://www.googleapis.com/auth/drive.readonly"];
        let token = self.auth_manager.get_token(scopes).await?;
        let result = self.drive.permissions().list(spreadsheet_id).doit().await;
        match result {
            Ok((_, perm_list)) => Ok(perm_list.permissions.unwrap_or_default()),
            Err(e) => {
                log_error(&format!("Failed to get permissions: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    pub async fn get_drive_file_metadata(&self, file_id: &str) -> Result<google_drive3::api::File, McpError> {
        log_info(&format!("Getting Drive file metadata for {}", file_id));
        let scopes = &["https://www.googleapis.com/auth/drive.readonly"];
        let token = self.auth_manager.get_token(scopes).await?;
        let result = self.drive.files().get(file_id).doit().await;
        match result {
            Ok((_, file)) => Ok(file),
            Err(e) => {
                log_error(&format!("Failed to get file metadata: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    pub async fn copy_spreadsheet(&self, spreadsheet_id: &str, new_title: &str) -> Result<String, McpError> {
        log_info(&format!("Copying spreadsheet {} as {}", spreadsheet_id, new_title));
        let scopes = &["https://www.googleapis.com/auth/drive"];
        let token = self.auth_manager.get_token(scopes).await?;
        let mut req = google_drive3::api::File::default();
        req.name = Some(new_title.to_string());
        req.mime_type = Some("application/vnd.google-apps.spreadsheet".to_string());
        let result = self.drive.files().copy(req, spreadsheet_id).doit().await;
        match result {
            Ok((_, file)) => Ok(file.id.unwrap_or_default()),
            Err(e) => {
                log_error(&format!("Failed to copy spreadsheet: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    pub async fn rename_spreadsheet(&self, spreadsheet_id: &str, new_title: &str) -> Result<(), McpError> {
        log_info(&format!("Renaming spreadsheet {} to {}", spreadsheet_id, new_title));
        let scopes = &["https://www.googleapis.com/auth/drive"];
        let _token = self.auth_manager.get_token(scopes).await?;
        let mut req = google_drive3::api::File::default();
        req.name = Some(new_title.to_string());
        let result = self.drive.files().update(req, spreadsheet_id)
            .param("fields", "*")
            .doit().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log_error(&format!("Failed to rename spreadsheet: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    pub async fn delete_spreadsheet(&self, spreadsheet_id: &str) -> Result<(), McpError> {
        log_info(&format!("Deleting spreadsheet {}", spreadsheet_id));
        let scopes = &["https://www.googleapis.com/auth/drive"];
        let token = self.auth_manager.get_token(scopes).await?;
        let result = self.drive.files().delete(spreadsheet_id).doit().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log_error(&format!("Failed to delete spreadsheet: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    pub async fn export_spreadsheet_pdf(&self, spreadsheet_id: &str) -> Result<Vec<u8>, McpError> {
        log_info(&format!("Exporting spreadsheet {} as PDF", spreadsheet_id));
        let scopes = &["https://www.googleapis.com/auth/drive.readonly"];
        let _token = self.auth_manager.get_token(scopes).await?;
        let result = self.drive.files().export(spreadsheet_id, "application/pdf").doit().await;
        match result {
            Ok(response) => {
                let bytes = google_sheets4::hyper::body::to_bytes(response.into_body()).await
                    .map_err(|e| McpError::ApiError(format!("Error reading export body: {}", e)))?;
                Ok(bytes.to_vec())
            },
            Err(e) => {
                log_error(&format!("Failed to export spreadsheet as PDF: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    pub async fn list_drive_folders(&self) -> Result<Vec<(String, String)>, McpError> {
        log_info("Listing Drive folders");
        let scopes = &["https://www.googleapis.com/auth/drive.readonly"];
        let token = self.auth_manager.get_token(scopes).await?;
        let result = self.drive.files().list()
            .q("mimeType='application/vnd.google-apps.folder'")
            .doit().await;
        match result {
            Ok((_, file_list)) => {
                let folders = file_list.files
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|f| {
                        match (f.id, f.name) {
                            (Some(id), Some(name)) => Some((id, name)),
                            _ => None,
                        }
                    })
                    .collect();
                Ok(folders)
            }
            Err(e) => {
                log_error(&format!("Failed to list folders: {}", e));
                Err(McpError::ApiError(format!("Drive API error: {}", e)))
            }
        }
    }

    // Add your methods to interact with Google Sheets and Drive here
}

// RealTokenProvider wraps AuthManager and implements GetToken for Google API clients
#[derive(Clone)]
pub struct RealTokenProvider {
    auth_manager: Arc<AuthManager>,
}

impl RealTokenProvider {
    pub fn new(auth_manager: Arc<AuthManager>) -> Self {
        RealTokenProvider { auth_manager }
    }
}

impl GetToken for RealTokenProvider {
    fn get_token<'a>(
        &'a self,
        scopes: &'a [&str],
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Option<String>, Box<dyn std::error::Error + Send + Sync>>> + Send + 'a>> {
        let auth_manager = self.auth_manager.clone();
        let scopes: Vec<String> = scopes.iter().map(|s| s.to_string()).collect();
        Box::pin(async move {
            let token_result = auth_manager.get_authenticator().token(&scopes).await;
            match token_result {
                Ok(token) => Ok(Some(token.token().expect("token missing").to_string())),
                Err(e) => Err(Box::new(McpError::Auth(format!("Token error: {}", e))) as Box<dyn std::error::Error + Send + Sync>)
            }
        })
    }
}