use axum::{routing::post, Router, Json};
use std::sync::Arc;
use crate::models::{
    McpHandler, EchoHandler, EchoRequest, EchoResponse, EchoError,
    ListSpreadsheetsHandler, ListSpreadsheetsRequest, ListSpreadsheetsResponse, SpreadsheetError,
    CreateSpreadsheetHandler, CreateSpreadsheetRequest, CreateSpreadsheetResponse
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

pub fn example_router(client: Arc<GoogleApiClient>) -> Router {
    Router::new()
        .route("/echo", post(echo_route))
        .route("/spreadsheets/list", post(list_spreadsheets_route))
        .route("/spreadsheets/create", post(create_spreadsheet_route))
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