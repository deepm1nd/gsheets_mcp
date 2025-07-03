use google_drive3::api::{DriveHub, Permission};
use google_sheets4::api::Sheets;
use google_docs1::api::Docs;
use google_calendar3::api::CalendarHub;
use yup_oauth2::{ServiceAccountAuthenticator, ServiceAccountKey, read_service_account_key};
use std::sync::Arc;
use tokio::sync::Mutex;
use hyper::Client;
use hyper_rustls::HttpsConnectorBuilder;

// Stub for AuthManager (replace with real import and implementation)
pub struct AuthManager;

// GoogleApiClient abstraction for Sheets and Drive
pub struct GoogleApiClient {
    pub drive: Arc<Mutex<DriveHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>>>,
    pub sheets: Arc<Mutex<Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>>>,
    pub docs: Arc<Mutex<Docs<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>>>,
    pub calendar: Arc<Mutex<CalendarHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>>>,
}

impl GoogleApiClient {
    pub async fn new(_auth_manager: Arc<AuthManager>) -> Self {
        // Load service account key from file (or env)
        let key = std::env::var("GOOGLE_SERVICE_ACCOUNT_KEY").expect("Set GOOGLE_SERVICE_ACCOUNT_KEY env var to path of service account JSON");
        let service_account_key = read_service_account_key(&key).await.expect("Failed to read service account key");
        let auth = ServiceAccountAuthenticator::builder(service_account_key)
            .build()
            .await
            .expect("Failed to create authenticator");
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .build();
        let client = Client::builder().build(connector);
        let drive = DriveHub::new(client.clone(), auth.clone());
        let sheets = Sheets::new(client.clone(), auth.clone());
        let docs = Docs::new(client.clone(), auth.clone());
        let calendar = CalendarHub::new(client, auth);
        GoogleApiClient {
            drive: Arc::new(Mutex::new(drive)),
            sheets: Arc::new(Mutex::new(sheets)),
            docs: Arc::new(Mutex::new(docs)),
            calendar: Arc::new(Mutex::new(calendar)),
        }
    }

    pub async fn list_spreadsheets(&self) -> Result<Vec<String>, String> {
        // Replace with real Google Drive API call
        Ok(vec!["Sheet1".to_string(), "Sheet2".to_string()])
    }

    pub async fn create_spreadsheet(&self, title: &str) -> Result<String, String> {
        // Replace with real Google Sheets API call
        Ok(format!("created-{}", title))
    }

    pub async fn get_sheet_data(&self, spreadsheet_id: &str, range: &str) -> Result<Vec<Vec<String>>, String> {
        // Replace with real Google Sheets API call
        Ok(vec![vec!["A1".to_string(), "B1".to_string()], vec!["A2".to_string(), "B2".to_string()]])
    }

    pub async fn set_sheet_data(&self, spreadsheet_id: &str, range: &str, values: Vec<Vec<String>>) -> Result<usize, String> {
        // Replace with real Google Sheets API call
        Ok(values.iter().map(|row| row.len()).sum()) // Return total cells updated
    }

    pub async fn batch_get_sheet_data(&self, spreadsheet_id: &str, ranges: &Vec<String>) -> Result<Vec<Vec<Vec<String>>>, String> {
        // Replace with real Google Sheets API call
        Ok(ranges.iter().map(|_| vec![vec!["A".to_string(), "B".to_string()]]).collect())
    }

    pub async fn batch_set_sheet_data(&self, spreadsheet_id: &str, updates: &Vec<(String, Vec<Vec<String>>)>) -> Result<usize, String> {
        // Replace with real Google Sheets API call
        Ok(updates.iter().map(|(_, values)| values.iter().map(|row| row.len()).sum::<usize>()).sum())
    }

    pub async fn list_drive_files(&self) -> Result<Vec<String>, String> {
        // Replace with real Google Drive API call
        Ok(vec!["file1_id".to_string(), "file2_id".to_string()])
    }

    pub async fn get_drive_file_metadata(&self, file_id: &str) -> Result<(String, String, Option<u64>), String> {
        // Replace with real Google Drive API call
        Ok((format!("File_{}", file_id), "application/vnd.google-apps.spreadsheet".to_string(), Some(1024)))
    }

    pub async fn list_permissions(&self, resource_id: &str, _resource_type: &crate::models::ResourceType) -> Result<Vec<crate::models::PermissionInfo>, String> {
        use google_drive3::api::Scope;
        let mut drive = self.drive.lock().await;
        let result = drive.permissions().list(resource_id).doit().await;
        match result {
            Ok((_, perms)) => {
                let permissions = perms.permissions.unwrap_or_default().into_iter().map(|p| crate::models::PermissionInfo {
                    id: p.id.unwrap_or_default(),
                    email: p.email_address,
                    role: p.role.unwrap_or_default(),
                    type_: p.type_.unwrap_or_default(),
                }).collect();
                Ok(permissions)
            },
            Err(e) => Err(format!("Drive API error: {e:?}")),
        }
    }

    pub async fn add_permission(&self, resource_id: &str, _resource_type: &crate::models::ResourceType, email: &str, role: &str, type_: &str) -> Result<String, String> {
        use google_drive3::api::Permission;
        let mut drive = self.drive.lock().await;
        let mut perm = Permission::default();
        perm.email_address = Some(email.to_string());
        perm.role = Some(role.to_string());
        perm.type_ = Some(type_.to_string());
        let result = drive.permissions().create(perm, resource_id).send_notification_email(false).doit().await;
        match result {
            Ok((_, p)) => Ok(p.id.unwrap_or_default()),
            Err(e) => Err(format!("Drive API error: {e:?}")),
        }
    }

    pub async fn remove_permission(&self, resource_id: &str, _resource_type: &crate::models::ResourceType, permission_id: &str) -> Result<bool, String> {
        let mut drive = self.drive.lock().await;
        let result = drive.permissions().delete(resource_id, permission_id).doit().await;
        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("Drive API error: {e:?}")),
        }
    }

    // List Google Docs files via Drive
    pub async fn list_docs(&self) -> Result<Vec<(String, String)>, anyhow::Error> {
        use google_drive3::api::Scope;
        let drive = self.drive.lock().await;
        let (_, file_list) = drive.files().list()
            .q("mimeType='application/vnd.google-apps.document'")
            .param("fields", "files(id,name)")
            .add_scope(Scope::Full)
            .doit().await?;
        let docs = file_list.files.unwrap_or_default().into_iter()
            .filter_map(|f| {
                match (f.id, f.name) {
                    (Some(id), Some(name)) => Some((id, name)),
                    _ => None
                }
            })
            .collect();
        Ok(docs)
    }
}
