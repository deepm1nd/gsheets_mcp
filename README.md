# gsheets_mcp

A robust, extensible Rust MCP Server to interact with Google Sheets and Google Drive, supporting HTTP, stdio, and SEE transports. Built for reliability, security, and easy integration with Google Workspace APIs.

## Features
- Full Google Sheets and Drive API support (CRUD, batch, sharing, metadata, export, etc.)
- Multi-transport: HTTP (REST/streaming), stdio (CLI/pipe), SEE (event-driven)
- Secure, flexible authentication: Service Account, OAuth2, Application Default Credentials (ADC)
- Structured logging, robust error handling, and layered configuration
- Modular, extensible architecture for future Google Workspace APIs

## Architecture
See `gsheets_mcp_architecture_specification.md` for a detailed breakdown.

- **MCP Server Core**: Multi-transport entrypoint
- **Google API Client**: Unified abstraction over Sheets/Drive
- **Authentication Manager**: Service Account, OAuth2, ADC
- **Error Handling & Logging**: Structured, actionable

## Configuration
Create a `config.toml` or use environment variables. Example:

```toml
[auth]
method = "service_account" # or "oauth2" or "adc"
service_account_key_path = "./service_account.json" # for service account
# oauth2_client_secret_path = "./client_secret.json" # for OAuth2
```

## Usage

### HTTP Server
Start the HTTP server (default: 127.0.0.1:8080):
```sh
cargo run --bin gsheets_mcp_http
```

#### Endpoints
| Method | Path | Description |
|--------|------|-------------|
| GET    | /spreadsheets | List spreadsheets |
| POST   | /spreadsheets | Create spreadsheet (`{"title": "My Sheet"}`) |
| GET    | /spreadsheets/:id | Get spreadsheet data |
| POST   | /spreadsheets/values | Set values (`spreadsheet_id`, `range`, `values`) |
| POST   | /spreadsheets/:id/copy | Copy spreadsheet |
| POST   | /spreadsheets/:id/rename | Rename spreadsheet |
| POST   | /spreadsheets/:id/delete | Delete spreadsheet |
| GET    | /spreadsheets/:id/export/pdf | Export as PDF |
| POST   | /spreadsheets/:id/batch_update | Batch update |
| GET    | /spreadsheets/:id/permissions | Get permissions |
| POST   | /spreadsheets/:id/share | Share spreadsheet |
| GET    | /drive/folders | List Drive folders |
| GET    | /drive/files/:id | Get Drive file metadata |

#### Response Format
All endpoints return:
```json
{
  "success": true/false,
  "data": ...,
  "error": "..." // if any
}
```

### Stdio Server
Read JSON commands from stdin, write JSON responses to stdout. Example:
```json
{"action": "list_spreadsheets"}
{"action": "create_spreadsheet", "title": "My Sheet"}
{"action": "set_sheet_values", "spreadsheet_id": "...", "range": "Sheet1!A1:B2", "values": [["A", "B"]]}
```

#### Supported Commands
- `list_spreadsheets`
- `create_spreadsheet` { title }
- `get_spreadsheet` { spreadsheet_id }
- `set_sheet_values` { spreadsheet_id, range, values }
- `batch_update` { spreadsheet_id, requests }
- `copy_spreadsheet` { spreadsheet_id, new_title }
- `rename_spreadsheet` { spreadsheet_id, new_title }
- `delete_spreadsheet` { spreadsheet_id }
- `export_spreadsheet_pdf` { spreadsheet_id }
- `list_drive_folders`
- `share_spreadsheet` { spreadsheet_id, email, role }
- `get_spreadsheet_permissions` { spreadsheet_id }
- `get_drive_file_metadata` { file_id }

#### Response Format
```json
{"success": true/false, "data": ..., "error": "..."}
```

### SEE Server
Event-driven/streaming: accepts JSON events, returns JSON responses. Example:
```json
{"event": "get_spreadsheet", "spreadsheet_id": "..."}
```

#### Supported Events
Same as stdio commands, but use `event` instead of `action`.

#### Response Format
```json
{"ok": true/false, "data": ..., "error": "..."}
```

## Authentication
- **Service Account**: Recommended for server/server and automation. Requires a Google Cloud service account JSON key.
- **OAuth2**: For user-driven flows. Requires OAuth2 client secret JSON.
- **ADC**: Uses gcloud or environment credentials if available.

## Extensibility
- Add new endpoints and Google Workspace APIs by extending `google_api_client.rs` and registering new handlers.
- Modular design for easy integration and testing.
- All transports (HTTP, stdio, SEE) use the same unified API client and error model.

## Error Handling & Security
- All errors are structured and actionable, with remediation hints in logs.
- Never log secrets or tokens.
- Validate all input and API responses.
- Follows least-privilege principle for credentials.
- All endpoints and commands are robustly validated and return clear error messages.

## Development & Testing
- See `gsheets_mcp_development_plan.md` for phased roadmap and checklist.
- Run all tests:
```sh
cargo test --all
```

## License
MIT or Apache-2.0 (choose and update as appropriate)

---
For full details, see the architecture and development plan markdown files in this repo.
