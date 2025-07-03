use crate::google_api_client::GoogleApiClient;
use std::sync::Arc;
use async_trait::async_trait;

/// Example request and response types
pub struct EchoRequest {
    pub message: String,
}

pub struct EchoResponse {
    pub echoed: String,
}

/// Example error type
#[derive(Debug)]
pub enum EchoError {
    EmptyMessage,
}

/// Example handler implementing the unified trait
pub struct EchoHandler;

#[async_trait]
impl McpHandler for EchoHandler {
    type Request = EchoRequest;
    type Response = EchoResponse;
    type Error = EchoError;

    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        if req.message.is_empty() {
            Err(EchoError::EmptyMessage)
        } else {
            Ok(EchoResponse { echoed: req.message })
        }
    }
}

// Request/response types for business logic
pub struct ListSpreadsheetsRequest;

pub struct ListSpreadsheetsResponse {
    pub spreadsheets: Vec<String>,
}

pub struct CreateSpreadsheetRequest {
    pub title: String,
}

pub struct CreateSpreadsheetResponse {
    pub id: String,
}

#[derive(Debug)]
pub enum SpreadsheetError {
    Api(String),
}

pub struct ListSpreadsheetsHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for ListSpreadsheetsHandler {
    type Request = ListSpreadsheetsRequest;
    type Response = ListSpreadsheetsResponse;
    type Error = SpreadsheetError;

    async fn handle(&self, _req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client
            .list_spreadsheets()
            .await
            .map(|sheets| ListSpreadsheetsResponse { spreadsheets: sheets })
            .map_err(SpreadsheetError::Api)
    }
}

pub struct CreateSpreadsheetHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for CreateSpreadsheetHandler {
    type Request = CreateSpreadsheetRequest;
    type Response = CreateSpreadsheetResponse;
    type Error = SpreadsheetError;

    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client
            .create_spreadsheet(&req.title)
            .await
            .map(|id| CreateSpreadsheetResponse { id })
            .map_err(SpreadsheetError::Api)
    }
}

// Get/Set sheet data types
pub struct GetSheetDataRequest {
    pub spreadsheet_id: String,
    pub range: String, // e.g., "Sheet1!A1:C10"
}

pub struct GetSheetDataResponse {
    pub values: Vec<Vec<String>>,
}

pub struct SetSheetDataRequest {
    pub spreadsheet_id: String,
    pub range: String,
    pub values: Vec<Vec<String>>,
}

pub struct SetSheetDataResponse {
    pub updated_cells: usize,
}

pub struct GetSheetDataHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for GetSheetDataHandler {
    type Request = GetSheetDataRequest;
    type Response = GetSheetDataResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.get_sheet_data(&req.spreadsheet_id, &req.range).await
            .map(|values| GetSheetDataResponse { values })
            .map_err(SpreadsheetError::Api)
    }
}

pub struct SetSheetDataHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for SetSheetDataHandler {
    type Request = SetSheetDataRequest;
    type Response = SetSheetDataResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.set_sheet_data(&req.spreadsheet_id, &req.range, req.values).await
            .map(|updated_cells| SetSheetDataResponse { updated_cells })
            .map_err(SpreadsheetError::Api)
    }
}

// Batch operations
pub struct BatchGetSheetDataRequest {
    pub spreadsheet_id: String,
    pub ranges: Vec<String>,
}

pub struct BatchGetSheetDataResponse {
    pub values: Vec<Vec<Vec<String>>>, // One Vec<Vec<String>> per range
}

pub struct BatchSetSheetDataRequest {
    pub spreadsheet_id: String,
    pub updates: Vec<(String, Vec<Vec<String>>)>, // (range, values)
}

pub struct BatchSetSheetDataResponse {
    pub updated_cells: usize,
}

pub struct BatchGetSheetDataHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for BatchGetSheetDataHandler {
    type Request = BatchGetSheetDataRequest;
    type Response = BatchGetSheetDataResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.batch_get_sheet_data(&req.spreadsheet_id, &req.ranges).await
            .map(|values| BatchGetSheetDataResponse { values })
            .map_err(SpreadsheetError::Api)
    }
}

pub struct BatchSetSheetDataHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for BatchSetSheetDataHandler {
    type Request = BatchSetSheetDataRequest;
    type Response = BatchSetSheetDataResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.batch_set_sheet_data(&req.spreadsheet_id, &req.updates).await
            .map(|updated_cells| BatchSetSheetDataResponse { updated_cells })
            .map_err(SpreadsheetError::Api)
    }
}

// Drive file operations
pub struct ListDriveFilesRequest;

pub struct ListDriveFilesResponse {
    pub files: Vec<String>,
}

pub struct GetDriveFileMetadataRequest {
    pub file_id: String,
}

pub struct GetDriveFileMetadataResponse {
    pub name: String,
    pub mime_type: String,
    pub size: Option<u64>,
}

pub struct ListDriveFilesHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for ListDriveFilesHandler {
    type Request = ListDriveFilesRequest;
    type Response = ListDriveFilesResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, _req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.list_drive_files().await
            .map(|files| ListDriveFilesResponse { files })
            .map_err(SpreadsheetError::Api)
    }
}

pub struct GetDriveFileMetadataHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for GetDriveFileMetadataHandler {
    type Request = GetDriveFileMetadataRequest;
    type Response = GetDriveFileMetadataResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.get_drive_file_metadata(&req.file_id).await
            .map(|(name, mime_type, size)| GetDriveFileMetadataResponse { name, mime_type, size })
            .map_err(SpreadsheetError::Api)
    }
}

// ===== Sharing/Permissions Types =====

#[derive(Debug)]
pub enum ResourceType {
    Spreadsheet,
    DriveFile,
}

pub struct ListPermissionsRequest {
    pub resource_id: String, // spreadsheet_id or file_id
    pub resource_type: ResourceType,
}

pub struct PermissionInfo {
    pub id: String,
    pub email: Option<String>,
    pub role: String,
    pub type_: String, // e.g., user, group, domain, anyone
}

pub struct ListPermissionsResponse {
    pub permissions: Vec<PermissionInfo>,
}

pub struct AddPermissionRequest {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub email: String,
    pub role: String, // e.g., reader, writer, owner
    pub type_: String, // e.g., user, group, domain, anyone
}

pub struct AddPermissionResponse {
    pub permission_id: String,
}

pub struct RemovePermissionRequest {
    pub resource_id: String,
    pub resource_type: ResourceType,
    pub permission_id: String,
}

pub struct RemovePermissionResponse {
    pub success: bool,
}

pub struct ListPermissionsHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for ListPermissionsHandler {
    type Request = ListPermissionsRequest;
    type Response = ListPermissionsResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.list_permissions(&req.resource_id, &req.resource_type).await
            .map(|permissions| ListPermissionsResponse { permissions })
            .map_err(SpreadsheetError::Api)
    }
}

pub struct AddPermissionHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for AddPermissionHandler {
    type Request = AddPermissionRequest;
    type Response = AddPermissionResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.add_permission(&req.resource_id, &req.resource_type, &req.email, &req.role, &req.type_).await
            .map(|permission_id| AddPermissionResponse { permission_id })
            .map_err(SpreadsheetError::Api)
    }
}

pub struct RemovePermissionHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for RemovePermissionHandler {
    type Request = RemovePermissionRequest;
    type Response = RemovePermissionResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.client.remove_permission(&req.resource_id, &req.resource_type, &req.permission_id).await
            .map(|success| RemovePermissionResponse { success })
            .map_err(SpreadsheetError::Api)
    }
}

// ===== Google Docs Types =====

pub struct GetDocumentRequest {
    pub document_id: String,
}

pub struct GetDocumentResponse {
    pub title: String,
    pub body: String, // Simplified: full text content
}

pub struct CreateDocumentRequest {
    pub title: String,
}

pub struct CreateDocumentResponse {
    pub document_id: String,
}

pub struct AppendTextRequest {
    pub document_id: String,
    pub text: String,
}

pub struct AppendTextResponse {
    pub success: bool,
}

#[derive(Debug)]
pub enum DocsError {
    Api(String),
}

pub struct GetDocumentHandler {
    pub client: Arc<GoogleApiClient>,
}

pub struct CreateDocumentHandler {
    pub client: Arc<GoogleApiClient>,
}

pub struct AppendTextHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for GetDocumentHandler {
    type Request = GetDocumentRequest;
    type Response = GetDocumentResponse;
    type Error = DocsError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        use google_docs1::api::Scope;
        let docs = self.client.docs.lock().await;
        let result = docs.documents().get(&req.document_id).doit().await;
        match result {
            Ok((_, doc)) => {
                let title = doc.title.unwrap_or_default();
                // Concatenate all text content from the document body
                let mut body = String::new();
                if let Some(content) = doc.body.and_then(|b| b.content) {
                    for el in content {
                        if let Some(para) = el.paragraph {
                            if let Some(elements) = para.elements {
                                for e in elements {
                                    if let Some(text_run) = e.text_run {
                                        if let Some(text) = text_run.content {
                                            body.push_str(&text);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(GetDocumentResponse { title, body })
            }
            Err(e) => Err(DocsError::Api(format!("Docs API error: {e:?}"))),
        }
    }
}

#[async_trait]
impl McpHandler for CreateDocumentHandler {
    type Request = CreateDocumentRequest;
    type Response = CreateDocumentResponse;
    type Error = DocsError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        let docs = self.client.docs.lock().await;
        let mut doc = google_docs1::api::Document::default();
        doc.title = Some(req.title);
        let result = docs.documents().create(doc).doit().await;
        match result {
            Ok((_, created)) => Ok(CreateDocumentResponse {
                document_id: created.document_id.unwrap_or_default(),
            }),
            Err(e) => Err(DocsError::Api(format!("Docs API error: {e:?}"))),
        }
    }
}

#[async_trait]
impl McpHandler for AppendTextHandler {
    type Request = AppendTextRequest;
    type Response = AppendTextResponse;
    type Error = DocsError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        let docs = self.client.docs.lock().await;
        // Insert text at the end of the document
        let requests = vec![
            google_docs1::api::Request {
                insert_text: Some(google_docs1::api::InsertTextRequest {
                    text: Some(req.text),
                    end_of_segment_location: Some(google_docs1::api::EndOfSegmentLocation { segment_id: None }),
                    location: None,
                }),
                ..Default::default()
            }
        ];
        let batch_req = google_docs1::api::BatchUpdateDocumentRequest { requests: Some(requests), ..Default::default() };
        let result = docs.documents().batch_update(batch_req, &req.document_id).doit().await;
        match result {
            Ok(_) => Ok(AppendTextResponse { success: true }),
            Err(e) => Err(DocsError::Api(format!("Docs API error: {e:?}"))),
        }
    }
}

// ListDocs request/response
#[derive(serde::Deserialize)]
pub struct ListDocsRequest {}

#[derive(serde::Serialize)]
pub struct ListDocsResponse {
    pub docs: Vec<DocInfo>,
}

#[derive(serde::Serialize)]
pub struct DocInfo {
    pub id: String,
    pub title: String,
}

// Handler for listing docs
pub struct ListDocsHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for ListDocsHandler {
    type Request = ListDocsRequest;
    type Response = ListDocsResponse;
    type Error = DocsError;

    async fn handle(&self, _req: Self::Request) -> Result<Self::Response, Self::Error> {
        let docs = self.client.list_docs().await.map_err(|e| DocsError::Api(e.to_string()))?;
        Ok(ListDocsResponse {
            docs: docs.into_iter().map(|(id, title)| DocInfo { id, title }).collect(),
        })
    }
}

// ==== Google Calendar types and handlers ====

#[derive(serde::Deserialize)]
pub struct GetCalendarEventRequest {
    pub calendar_id: String,
    pub event_id: String,
}

#[derive(serde::Serialize)]
pub struct GetCalendarEventResponse {
    pub id: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct CreateCalendarEventRequest {
    pub calendar_id: String,
    pub summary: String,
    pub description: Option<String>,
    pub start: String, // RFC3339
    pub end: String,   // RFC3339
}

#[derive(serde::Serialize)]
pub struct CreateCalendarEventResponse {
    pub id: String,
}

#[derive(serde::Deserialize)]
pub struct ListCalendarEventsRequest {
    pub calendar_id: String,
}

#[derive(serde::Serialize)]
pub struct CalendarEventInfo {
    pub id: String,
    pub summary: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(serde::Serialize)]
pub struct ListCalendarEventsResponse {
    pub events: Vec<CalendarEventInfo>,
}

// Error type for Calendar handlers
#[derive(Debug)]
pub enum CalendarError {
    Api(String),
}

// Handler stubs
pub struct GetCalendarEventHandler {
    pub client: Arc<GoogleApiClient>,
}

pub struct CreateCalendarEventHandler {
    pub client: Arc<GoogleApiClient>,
}

pub struct ListCalendarEventsHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for GetCalendarEventHandler {
    type Request = GetCalendarEventRequest;
    type Response = GetCalendarEventResponse;
    type Error = CalendarError;

    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        use google_calendar3::api::Scope;
        let calendar = self.client.calendar.lock().await;
        let (_, event) = calendar.events().get(&req.calendar_id, &req.event_id)
            .add_scope(Scope::Full)
            .doit().await.map_err(|e| CalendarError::Api(e.to_string()))?;
        Ok(GetCalendarEventResponse {
            id: event.id.unwrap_or_default(),
            summary: event.summary,
            description: event.description,
            start: event.start.and_then(|s| s.date_time.map(|dt| dt.to_rfc3339())),
            end: event.end.and_then(|e| e.date_time.map(|dt| dt.to_rfc3339())),
        })
    }
}

#[async_trait]
impl McpHandler for CreateCalendarEventHandler {
    type Request = CreateCalendarEventRequest;
    type Response = CreateCalendarEventResponse;
    type Error = CalendarError;

    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        use google_calendar3::api::{Event, EventDateTime, Scope};
        let mut event = Event::default();
        event.summary = Some(req.summary);
        event.description = req.description;
        use google_calendar3::chrono;
        event.start = Some(EventDateTime {
            date_time: Some(req.start.parse::<chrono::DateTime<chrono::Utc>>().map_err(|e| CalendarError::Api(e.to_string()))?),
            ..Default::default()
        });
        event.end = Some(EventDateTime {
            date_time: Some(req.end.parse::<chrono::DateTime<chrono::Utc>>().map_err(|e| CalendarError::Api(e.to_string()))?),
            ..Default::default()
        });
        let calendar = self.client.calendar.lock().await;
        let (_, created) = calendar.events().insert(event, &req.calendar_id)
            .add_scope(Scope::Full)
            .doit().await.map_err(|e| CalendarError::Api(e.to_string()))?;
        Ok(CreateCalendarEventResponse {
            id: created.id.unwrap_or_default(),
        })
    }
}

#[async_trait]
impl McpHandler for ListCalendarEventsHandler {
    type Request = ListCalendarEventsRequest;
    type Response = ListCalendarEventsResponse;
    type Error = CalendarError;

    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        use google_calendar3::api::Scope;
        let calendar = self.client.calendar.lock().await;
        let (_, events) = calendar.events().list(&req.calendar_id)
            .add_scope(Scope::Full)
            .doit().await.map_err(|e| CalendarError::Api(e.to_string()))?;
        let items = events.items.unwrap_or_default();
        let result = items.into_iter().map(|event| CalendarEventInfo {
            id: event.id.unwrap_or_default(),
            summary: event.summary,
            start: event.start.and_then(|s| s.date_time.map(|dt| dt.to_rfc3339())),
            end: event.end.and_then(|e| e.date_time.map(|dt| dt.to_rfc3339())),
        }).collect();
        Ok(ListCalendarEventsResponse { events: result })
    }
}

#[async_trait]
pub trait McpHandler {
    type Request;
    type Response;
    type Error;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
}
