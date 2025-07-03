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

#[async_trait]
pub trait McpHandler {
    type Request;
    type Response;
    type Error;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
}
