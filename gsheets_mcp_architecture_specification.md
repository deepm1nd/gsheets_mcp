# 1. gsheets_mcp Architecture Specification

## 1.1 Overview
This document defines the architecture for a robust, extensible Rust MCP server for Google Sheets (and potentially other Google Workspace apps), inspired by the reference implementation in `xing5/mcp-google-sheets` and following the requirements in `AGENTS.md`.

## 1.2 Goals
- Provide a Model Context Protocol (MCP) server in Rust for Google Sheets.
- Support a wide range of Google Sheets operations: CRUD, batch, sharing, metadata, and more.
- Enable secure, flexible authentication (Service Account, OAuth2, ADC).
- Be maximally robust, complete, and maintainable.
- Allow for future extension to other Google Workspace APIs (Drive, Docs, etc.).

## 1.3 High-Level Architecture
- **Language:** Rust
- **Project Structure:**
  - `src/` — All Rust source code
  - `prompts/` — Task prompts (future)
  - Root — Documentation, plans, and specifications
- **Core Components:**
  1. **MCP Server Core**: Multi-transport server exposing MCP endpoints via:
     - HTTP (REST and streaming, e.g., with `axum`)
     - stdio (for CLI/pipe-based integration)
     - SEE (Streaming Event Exchange, for event-driven/streaming clients)
  2. **Google API Client Layer**: Abstraction over Google Sheets/Drive APIs
  3. **Authentication Manager**: Handles Service Account, OAuth2, ADC
  4. **Task/Prompt Handler**: (Planned) For AI-driven task execution
  5. **Error Handling & Logging**: Robust, detailed error and event logging

## 1.4 Key Interfaces & APIs
- **MCP Protocol**: Define endpoints for all supported operations (list, create, read, update, batch, share, etc.)
- **Google Sheets API**: Use `google-apis-rs` or REST/gRPC as available
- **Authentication**: Support for Service Account (JSON), OAuth2, ADC

## 1.5 Authentication Flow
- Try Service Account (env/config)
- Fallback to OAuth2 (env/config)
- Fallback to ADC (env/config or gcloud)
- Fail with robust error if all methods unavailable

## 1.6 Error Handling
- All errors must be logged and returned with actionable messages
- Use Rust's `Result` and custom error types

## 1.7 Logging
- Use structured logging (e.g., `tracing` crate)
- Log all requests, responses, errors, and authentication events

## 1.8 Extensibility
- Modular design for easy addition of new Google Workspace APIs
- Clear separation of concerns between protocol, API, and auth layers

## 1.9 Security
- Never log secrets or sensitive tokens
- Validate all user input and API responses
- Support least-privilege principle for credentials

## 1.10 Build & Test
- Build early, build often: CI/CD ready
- Unit and integration tests for all modules

## 1.11 Technology Options
- **HTTP Server:** `axum` (preferred), `warp`, or `actix-web`
- **Google API Client:** `google-apis-rs` (preferred), `yup-oauth2`, or direct REST
- **Serialization:** `serde`, `serde_json`
- **Logging:** `tracing`, `log`
- **Testing:** `tokio::test`, `assert`, `mockito` for HTTP mocks

## 1.12 Diagrams
- (Planned) Add system and data flow diagrams

---

## 2. Technical Architecture

### 2.1 Module Breakdown
- **mcp_server**: Multi-transport server exposing MCP endpoints (main entrypoint)
  - **http_server**: HTTP/REST and HTTP streaming (axum/warp/actix-web)
  - **stdio_server**: stdio-based protocol handler (for CLI/pipe)
  - **see_server**: SEE (Streaming Event Exchange) protocol handler
- **google_api_client**: Abstraction over Google Sheets/Drive APIs
- **auth_manager**: Handles Service Account, OAuth2, ADC authentication
- **models**: Data structures for requests, responses, and Google API objects
- **error**: Custom error types and error handling utilities
- **logging**: Structured logging setup and macros
- **config**: Environment/config file parsing and validation
- **tests**: Unit and integration tests for all modules

### 2.2 Key Interfaces (Rust Traits/Structs)

#### MCP Server Transports
- **HTTP Example:**
```rust
// src/http_server.rs
use axum::{Router, routing::post, extract::Json};
use crate::models::*;

pub fn app_router() -> Router {
    Router::new()
        .route("/list_spreadsheets", post(list_spreadsheets))
        .route("/create_spreadsheet", post(create_spreadsheet))
        .route("/get_sheet_data", post(get_sheet_data))
        // ...other endpoints
}

async fn list_spreadsheets(Json(req): Json<ListSpreadsheetsRequest>) -> Json<ListSpreadsheetsResponse> {
    // ...implementation
}
```
- **Stdio Example:**
```rust
// src/stdio_server.rs
pub fn run_stdio_server() {
    // Read requests from stdin, write responses to stdout
    // ...implementation
}
```
- **SEE Example:**
```rust
// src/see_server.rs
pub fn run_see_server() {
    // Handle SEE protocol (event-driven streaming)
    // ...implementation
}
```

#### Transport-Agnostic Handler Trait
```rust
pub trait McpTransport {
    fn start(&self) -> Result<(), GsheetsError>;
}
```

#### Google API Client Abstraction
```rust
// src/google_api_client.rs
pub trait GoogleSheetsClient {
    fn list_spreadsheets(&self) -> Result<Vec<SpreadsheetInfo>, GsheetsError>;
    fn create_spreadsheet(&self, title: &str) -> Result<SpreadsheetInfo, GsheetsError>;
    fn get_sheet_data(&self, spreadsheet_id: &str, sheet: &str, range: Option<&str>) -> Result<SheetData, GsheetsError>;
    // ...other methods
}
```

#### Authentication Manager
```rust
// src/auth_manager.rs
pub enum AuthMethod {
    ServiceAccount(String), // path to JSON
    OAuth2 { client_id: String, client_secret: String, token_path: String },
    ApplicationDefault,
}

pub struct AuthManager { /* ... */ }
impl AuthManager {
    pub fn authenticate(&self) -> Result<GoogleAuth, AuthError>;
}
```

#### Error Handling
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GsheetsError {
    #[error("Google API error: {0}")]
    GoogleApi(String),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    // ...other variants
}
```

### 2.3 Dependencies
- **HTTP Server:** `axum` (preferred for REST/streaming)
- **Stdio:** `tokio`/`async-std` for async I/O
- **SEE:** Custom or community crate for SEE protocol (define protocol if not available)
- **Google API Client:** `google-apis-rs` (preferred: `google-sheets4`, `google-drive3`), `yup-oauth2` for auth
- **Serialization:** `serde`, `serde_json`
- **Logging:** `tracing`, `tracing-subscriber`
- **Error Handling:** `thiserror`, `anyhow`
- **Config:** `config`, `dotenv`
- **Testing:** `tokio`, `mockito`, `assert_matches`

### 2.4 Example Data Structures
```rust
// src/models.rs
#[derive(Serialize, Deserialize)]
pub struct ListSpreadsheetsRequest {
    pub folder_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SpreadsheetInfo {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct SheetData {
    pub values: Vec<Vec<String>>,
    pub range: String,
}
```

### 2.5 Authentication Flow (Detailed)
1. **Service Account**: Load credentials from JSON file, use `yup-oauth2` to obtain token.
2. **OAuth2**: Use client ID/secret, interactive flow if needed, store token in file.
3. **ADC**: Use `GOOGLE_APPLICATION_CREDENTIALS` or gcloud config.
4. **Fallback**: Fail with actionable error if all methods unavailable.

### 2.6 Error Handling & Logging
- All API and internal errors must be wrapped in custom error types and logged with context.
- Use `tracing` for structured logs (request IDs, user, operation, error details).
- Return actionable error messages to clients, never leak secrets.

### 2.7 Extensibility & Modularity
- Each Google API (Sheets, Drive, Docs) in its own module, sharing common auth and error code.
- MCP endpoints are thin wrappers over the API client layer.
- New endpoints can be added by implementing trait methods and exposing new routes.

### 2.8 Security Considerations
- Never log or return secrets, tokens, or sensitive config.
- Validate all incoming requests (types, bounds, required fields).
- Use least-privilege credentials and restrict API scopes.
- Support for rotating credentials and revoking tokens.

### 2.9 Build, Test, and CI
- Use `cargo test` for all modules, with unit and integration tests.
- Use `mockito` to mock Google API responses in tests.
- Provide a `Makefile` or CI config for automated build/test.

### 2.10 Example Directory Structure
```
/gsheets_mcp
  /src
    main.rs (or lib.rs)
    mcp_server.rs
    http_server.rs
    stdio_server.rs
    see_server.rs
    google_api_client.rs
    auth_manager.rs
    models.rs
    error.rs
    logging.rs
    config.rs
    tests/
  /prompts
  gsheets_mcp_architecture_specification.md
  gsheets_mcp_development_plan.md
  README.md
  Cargo.toml
```

---

This document will be iteratively updated as the project evolves. All implementation must strictly follow this architecture and the requirements in `AGENTS.md`.
