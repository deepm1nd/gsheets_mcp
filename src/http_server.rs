use axum::{routing::post, Router, Json};
use std::sync::Arc;
use crate::models::{
    McpHandler, EchoHandler, EchoRequest, EchoResponse, EchoError,
    ListSpreadsheetsHandler, ListSpreadsheetsRequest, ListSpreadsheetsResponse, SpreadsheetError,
    CreateSpreadsheetHandler, CreateSpreadsheetRequest, CreateSpreadsheetResponse,
    GetSheetDataHandler, GetSheetDataRequest, GetSheetDataResponse,
    SetSheetDataHandler, SetSheetDataRequest, SetSheetDataResponse,
    BatchGetSheetDataHandler, BatchGetSheetDataRequest, BatchGetSheetDataResponse,
    BatchSetSheetDataHandler, BatchSetSheetDataRequest, BatchSetSheetDataResponse,
    ListDriveFilesHandler, ListDriveFilesRequest, ListDriveFilesResponse,
    GetDriveFileMetadataHandler, GetDriveFileMetadataRequest, GetDriveFileMetadataResponse,
    ListPermissionsHandler, ListPermissionsRequest, ListPermissionsResponse,
    AddPermissionHandler, AddPermissionRequest, AddPermissionResponse,
    RemovePermissionHandler, RemovePermissionRequest, RemovePermissionResponse,
    GetDocumentHandler, GetDocumentRequest, GetDocumentResponse, DocsError,
    CreateDocumentHandler, CreateDocumentRequest, CreateDocumentResponse,
    AppendTextHandler, AppendTextRequest, AppendTextResponse,
    ListDocsHandler, ListDocsRequest, ListDocsResponse,
    GetCalendarEventHandler, CreateCalendarEventHandler, ListCalendarEventsHandler,
};
use crate::google_api_client::GoogleApiClient;

async fn echo_route(Json(req): Json<EchoRequest>) -> Result<Json<EchoResponse>, String> {
    let handler = EchoHandler;
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("Echo error: {:?}", e))
}

async fn list_spreadsheets_route(
    Json(_): Json<ListSpreadsheetsRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<ListSpreadsheetsResponse>, String> {
    let handler = ListSpreadsheetsHandler { client };
    handler.handle(ListSpreadsheetsRequest).await
        .map(Json)
        .map_err(|e| format!("List error: {:?}", e))
}

async fn create_spreadsheet_route(
    Json(req): Json<CreateSpreadsheetRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<CreateSpreadsheetResponse>, String> {
    let handler = CreateSpreadsheetHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("Create error: {:?}", e))
}

async fn get_sheet_data_route(
    Json(req): Json<GetSheetDataRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<GetSheetDataResponse>, String> {
    let handler = GetSheetDataHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("GetSheetData error: {:?}", e))
}

async fn set_sheet_data_route(
    Json(req): Json<SetSheetDataRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<SetSheetDataResponse>, String> {
    let handler = SetSheetDataHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("SetSheetData error: {:?}", e))
}

async fn batch_get_sheet_data_route(
    Json(req): Json<BatchGetSheetDataRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<BatchGetSheetDataResponse>, String> {
    let handler = BatchGetSheetDataHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("BatchGetSheetData error: {:?}", e))
}

async fn batch_set_sheet_data_route(
    Json(req): Json<BatchSetSheetDataRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<BatchSetSheetDataResponse>, String> {
    let handler = BatchSetSheetDataHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("BatchSetSheetData error: {:?}", e))
}

async fn list_drive_files_route(
    Json(_): Json<ListDriveFilesRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<ListDriveFilesResponse>, String> {
    let handler = ListDriveFilesHandler { client };
    handler.handle(ListDriveFilesRequest).await
        .map(Json)
        .map_err(|e| format!("ListDriveFiles error: {:?}", e))
}

async fn get_drive_file_metadata_route(
    Json(req): Json<GetDriveFileMetadataRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<GetDriveFileMetadataResponse>, String> {
    let handler = GetDriveFileMetadataHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("GetDriveFileMetadata error: {:?}", e))
}

async fn list_permissions_route(
    Json(req): Json<ListPermissionsRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<ListPermissionsResponse>, String> {
    let handler = ListPermissionsHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("ListPermissions error: {:?}", e))
}

async fn add_permission_route(
    Json(req): Json<AddPermissionRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<AddPermissionResponse>, String> {
    let handler = AddPermissionHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("AddPermission error: {:?}", e))
}

async fn remove_permission_route(
    Json(req): Json<RemovePermissionRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<RemovePermissionResponse>, String> {
    let handler = RemovePermissionHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("RemovePermission error: {:?}", e))
}

async fn get_document_route(
    Json(req): Json<GetDocumentRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<GetDocumentResponse>, String> {
    let handler = GetDocumentHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("GetDocument error: {:?}", e))
}

async fn create_document_route(
    Json(req): Json<CreateDocumentRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<CreateDocumentResponse>, String> {
    let handler = CreateDocumentHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("CreateDocument error: {:?}", e))
}

async fn append_text_route(
    Json(req): Json<AppendTextRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<AppendTextResponse>, String> {
    let handler = AppendTextHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("AppendText error: {:?}", e))
}

async fn list_docs_route(
    Json(_): Json<ListDocsRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<ListDocsResponse>, String> {
    let handler = ListDocsHandler { client };
    handler.handle(ListDocsRequest).await
        .map(Json)
        .map_err(|e| format!("ListDocs error: {:?}", e))
}

async fn get_calendar_event_route(
    Json(req): Json<GetCalendarEventRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<GetCalendarEventResponse>, String> {
    let handler = GetCalendarEventHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("GetCalendarEvent error: {:?}", e))
}

async fn create_calendar_event_route(
    Json(req): Json<CreateCalendarEventRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<CreateCalendarEventResponse>, String> {
    let handler = CreateCalendarEventHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("CreateCalendarEvent error: {:?}", e))
}

async fn list_calendar_events_route(
    Json(req): Json<ListCalendarEventsRequest>,
    axum::extract::Extension(client): axum::extract::Extension<Arc<GoogleApiClient>>,
) -> Result<Json<ListCalendarEventsResponse>, String> {
    let handler = ListCalendarEventsHandler { client };
    handler.handle(req).await
        .map(Json)
        .map_err(|e| format!("ListCalendarEvents error: {:?}", e))
}

pub fn example_router(client: Arc<GoogleApiClient>) -> Router {
    Router::new()
        .route("/echo", post(echo_route))
        .route("/spreadsheets/list", post(list_spreadsheets_route))
        .route("/spreadsheets/create", post(create_spreadsheet_route))
        .route("/sheets/get", post(get_sheet_data_route))
        .route("/sheets/set", post(set_sheet_data_route))
        .route("/sheets/batch_get", post(batch_get_sheet_data_route))
        .route("/sheets/batch_set", post(batch_set_sheet_data_route))
        .route("/drive/list", post(list_drive_files_route))
        .route("/drive/metadata", post(get_drive_file_metadata_route))
        .route("/drive/permissions/list", post(list_permissions_route))
        .route("/drive/permissions/add", post(add_permission_route))
        .route("/drive/permissions/remove", post(remove_permission_route))
        .route("/docs/get", post(get_document_route))
        .route("/docs/create", post(create_document_route))
        .route("/docs/append", post(append_text_route))
        .route("/docs/list", post(list_docs_route))
        .route("/calendar/get_event", post(get_calendar_event_route))
        .route("/calendar/create_event", post(create_calendar_event_route))
        .route("/calendar/list_events", post(list_calendar_events_route))
        .layer(axum::extract::Extension(client))
}

// Example: Integrate the unified handler into a full Axum server
#[tokio::main]
pub async fn main() {
    let client = Arc::new(GoogleApiClient::new(Arc::new(crate::google_api_client::AuthManager)).await);
    let app = example_router(client);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}