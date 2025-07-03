// HTTP server implementation using axum
use std::sync::Arc;
use crate::google_api_client::GoogleApiClient;
use crate::error::McpError;
use axum::{Router, routing::get, routing::post, Json, extract::{State, Path}};
use axum::body::Body;
use axum::http::Response;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use crate::{log_request, log_response};

#[derive(Deserialize, Debug)]
pub struct CreateSpreadsheetRequest {
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct SetSheetValuesRequest {
    pub spreadsheet_id: String,
    pub range: String,
    pub values: Vec<Vec<String>>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

pub struct HttpServer {
    pub api_client: Arc<GoogleApiClient>,
}

impl HttpServer {
    pub fn new(api_client: Arc<GoogleApiClient>) -> Self {
        HttpServer { api_client }
    }

    pub async fn run(&self, addr: &str) -> Result<(), McpError> {
        let api_client = self.api_client.clone();
        let app = Router::new()
            .route("/spreadsheets", get(list_spreadsheets_handler))
            .route("/spreadsheets", post(create_spreadsheet_handler))
            .route("/spreadsheets/:id", get(get_spreadsheet_handler))
            .route("/spreadsheets/values", post(set_sheet_values_handler))
            .route("/spreadsheets/:id/copy", post(copy_spreadsheet_handler))
            .route("/spreadsheets/:id/rename", post(rename_spreadsheet_handler))
            .route("/spreadsheets/:id/delete", post(delete_spreadsheet_handler))
            .route("/spreadsheets/:id/export/pdf", get(export_spreadsheet_pdf_handler))
            .route("/drive/folders", get(list_drive_folders_handler))
            .route("/spreadsheets/:id/share", post(share_spreadsheet_handler))
            .route("/spreadsheets/:id/permissions", get(get_spreadsheet_permissions_handler))
            .route("/spreadsheets/:id/batch_update", post(batch_update_handler))
            .route("/drive/files/:id", get(get_drive_file_metadata_handler))
            .route("/drive/files/:id/metadata", get(get_drive_file_metadata_handler))
            .with_state(api_client.clone());
        let addr: SocketAddr = addr.parse().unwrap_or_else(|_| "127.0.0.1:8080".parse().unwrap());
        axum::serve(tokio::net::TcpListener::bind(addr).await?, app.into_make_service())
            .await
            .map_err(|e| McpError::Other(format!("HTTP server error: {}", e)))
    }
}

#[derive(Deserialize, Debug)]
pub struct RenameRequest {
    pub new_title: String,
}

#[derive(Deserialize, Debug)]
pub struct ShareRequest {
    pub email: String,
    pub role: String,
}

#[derive(Deserialize, Debug)]
pub struct BatchUpdateRequest {
    pub requests: Vec<serde_json::Value>, // Accepts raw JSON for batch requests
}

// Handler for listing spreadsheets
async fn list_spreadsheets_handler(State(api_client): State<Arc<GoogleApiClient>>) -> Json<ApiResponse<Vec<String>>> {
    log_request!("list_spreadsheets", "");
    let result = api_client.list_spreadsheets().await;
    log_response!("list_spreadsheets", &result);
    match result {
        Ok(list) => Json(ApiResponse { success: true, data: Some(list), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for creating a spreadsheet
async fn create_spreadsheet_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Json(req): Json<CreateSpreadsheetRequest>,
) -> Json<ApiResponse<String>> {
    log_request!("create_spreadsheet", &req);
    let result = api_client.create_spreadsheet(&req.title).await;
    log_response!("create_spreadsheet", &result);
    match result {
        Ok(id) => Json(ApiResponse { success: true, data: Some(id), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for getting a spreadsheet
async fn get_spreadsheet_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    log_request!("get_spreadsheet", &id);
    let result = api_client.get_spreadsheet(&id).await;
    log_response!("get_spreadsheet", &result);
    match result {
        Ok(sheet) => Json(ApiResponse { success: true, data: Some(serde_json::to_value(sheet).unwrap_or_default()), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for setting sheet values
async fn set_sheet_values_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Json(req): Json<SetSheetValuesRequest>,
) -> Json<ApiResponse<()>> {
    log_request!("set_sheet_values", &req);
    let result = api_client.set_sheet_values(&req.spreadsheet_id, &req.range, req.values).await;
    log_response!("set_sheet_values", &result);
    match result {
        Ok(_) => Json(ApiResponse { success: true, data: None, error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for copying a spreadsheet
async fn copy_spreadsheet_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
    Json(req): Json<RenameRequest>,
) -> Json<ApiResponse<String>> {
    log_request!("copy_spreadsheet", &req);
    let result = api_client.copy_spreadsheet(&id, &req.new_title).await;
    log_response!("copy_spreadsheet", &result);
    match result {
        Ok(new_id) => Json(ApiResponse { success: true, data: Some(new_id), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for renaming a spreadsheet
async fn rename_spreadsheet_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
    Json(req): Json<RenameRequest>,
) -> Json<ApiResponse<()>> {
    log_request!("rename_spreadsheet", &req);
    let result = api_client.rename_spreadsheet(&id, &req.new_title).await;
    log_response!("rename_spreadsheet", &result);
    match result {
        Ok(_) => Json(ApiResponse { success: true, data: None, error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for deleting a spreadsheet
async fn delete_spreadsheet_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<()>> {
    log_request!("delete_spreadsheet", &id);
    let result = api_client.delete_spreadsheet(&id).await;
    log_response!("delete_spreadsheet", &result);
    match result {
        Ok(_) => Json(ApiResponse { success: true, data: None, error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for exporting a spreadsheet as PDF
async fn export_spreadsheet_pdf_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
) -> Response<Body> {
    match api_client.export_spreadsheet_pdf(&id).await {
        Ok(bytes) => Response::builder()
            .header("Content-Type", "application/pdf")
            .body(Body::from(bytes))
            .unwrap(),
        Err(e) => Response::builder()
            .status(500)
            .body(Body::from(format!("Error: {}", e)))
            .unwrap(),
    }
}

// Handler for listing drive folders
async fn list_drive_folders_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
) -> Json<ApiResponse<Vec<(String, String)>>> {
    log_request!("list_drive_folders", "");
    let result = api_client.list_drive_folders().await;
    log_response!("list_drive_folders", &result);
    match result {
        Ok(folders) => Json(ApiResponse { success: true, data: Some(folders), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for sharing a spreadsheet
async fn share_spreadsheet_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
    Json(req): Json<ShareRequest>,
) -> Json<ApiResponse<()>> {
    let result = api_client.share_spreadsheet(&id, &req.email, &req.role).await;
    match result {
        Ok(_) => Json(ApiResponse { success: true, data: None, error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for getting spreadsheet permissions
async fn get_spreadsheet_permissions_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    let result = api_client.get_spreadsheet_permissions(&id).await;
    match result {
        Ok(perms) => Json(ApiResponse { success: true, data: Some(serde_json::to_value(perms).unwrap_or_default()), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for batch updating a spreadsheet
async fn batch_update_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
    Json(req): Json<BatchUpdateRequest>,
) -> Json<ApiResponse<()>> {
    let requests: Vec<google_sheets4::api::Request> = req.requests.into_iter().filter_map(|v| serde_json::from_value(v).ok()).collect();
    let result = api_client.batch_update(&id, requests).await;
    match result {
        Ok(_) => Json(ApiResponse { success: true, data: None, error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}

// Handler for getting drive file metadata
async fn get_drive_file_metadata_handler(
    State(api_client): State<Arc<GoogleApiClient>>,
    Path(id): Path<String>,
) -> Json<ApiResponse<serde_json::Value>> {
    let result = api_client.get_drive_file_metadata(&id).await;
    match result {
        Ok(file) => Json(ApiResponse { success: true, data: Some(serde_json::to_value(file).unwrap_or_default()), error: None }),
        Err(e) => Json(ApiResponse { success: false, data: None, error: Some(e.to_string()) }),
    }
}
