use std::sync::Arc;
use crate::models::{McpHandler, ListSpreadsheetsHandler, ListSpreadsheetsRequest, CreateSpreadsheetHandler, CreateSpreadsheetRequest,
    GetSheetDataHandler, GetSheetDataRequest, SetSheetDataHandler, SetSheetDataRequest,
    BatchGetSheetDataHandler, BatchGetSheetDataRequest, BatchSetSheetDataHandler, BatchSetSheetDataRequest,
    ListDriveFilesHandler, ListDriveFilesRequest, GetDriveFileMetadataHandler, GetDriveFileMetadataRequest,
    ListPermissionsHandler, ListPermissionsRequest, AddPermissionHandler, AddPermissionRequest, RemovePermissionHandler, RemovePermissionRequest,
    GetDocumentHandler, GetDocumentRequest, CreateDocumentHandler, CreateDocumentRequest,
    AppendTextHandler, AppendTextRequest, GetCalendarEventHandler, GetCalendarEventRequest, CreateCalendarEventHandler, CreateCalendarEventRequest,
    ListCalendarEventsHandler, ListCalendarEventsRequest};
use crate::google_api_client::GoogleApiClient;

pub async fn stdio_server_main(client: Arc<GoogleApiClient>) {
    use std::io::{self, BufRead, Write};
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "list" {
            let handler = ListSpreadsheetsHandler { client: client.clone() };
            let result = handler.handle(ListSpreadsheetsRequest).await;
            writeln!(stdout, "{:?}", result).unwrap();
        } else if line.starts_with("create ") {
            let title = line[7..].to_string();
            let handler = CreateSpreadsheetHandler { client: client.clone() };
            let req = CreateSpreadsheetRequest { title };
            let result = handler.handle(req).await;
            writeln!(stdout, "{:?}", result).unwrap();
        } else if line.starts_with("get ") {
            // get <spreadsheet_id> <range>
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let req = GetSheetDataRequest {
                    spreadsheet_id: parts[1].to_string(),
                    range: parts[2].to_string(),
                };
                let handler = GetSheetDataHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: get <spreadsheet_id> <range>").unwrap();
            }
        } else if line.starts_with("set ") {
            // set <spreadsheet_id> <range> <csv_values>
            let parts: Vec<&str> = line.splitn(4, ' ').collect();
            if parts.len() == 4 {
                let values: Vec<Vec<String>> = parts[3]
                    .split(';')
                    .map(|row| row.split(',').map(|s| s.to_string()).collect())
                    .collect();
                let req = SetSheetDataRequest {
                    spreadsheet_id: parts[1].to_string(),
                    range: parts[2].to_string(),
                    values,
                };
                let handler = SetSheetDataHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: set <spreadsheet_id> <range> <csv_values>").unwrap();
            }
        } else if line.starts_with("batch_get ") {
            // batch_get <spreadsheet_id> <range1,range2,...>
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let ranges: Vec<String> = parts[2].split(',').map(|s| s.to_string()).collect();
                let req = BatchGetSheetDataRequest {
                    spreadsheet_id: parts[1].to_string(),
                    ranges,
                };
                let handler = BatchGetSheetDataHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: batch_get <spreadsheet_id> <range1,range2,...>").unwrap();
            }
        } else if line.starts_with("batch_set ") {
            // batch_set <spreadsheet_id> <range1:csv1;range2:csv2;...>
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let updates: Vec<(String, Vec<Vec<String>>)> = parts[2]
                    .split(';')
                    .filter_map(|pair| {
                        let mut split = pair.splitn(2, ':');
                        let range = split.next()?.to_string();
                        let csv = split.next()?;
                        let values = csv.split('|').map(|row| row.split(',').map(|s| s.to_string()).collect()).collect();
                        Some((range, values))
                    })
                    .collect();
                let req = BatchSetSheetDataRequest {
                    spreadsheet_id: parts[1].to_string(),
                    updates,
                };
                let handler = BatchSetSheetDataHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: batch_set <spreadsheet_id> <range1:csv1;range2:csv2;...>").unwrap();
            }
        } else if line == "drive_list" {
            let handler = ListDriveFilesHandler { client: client.clone() };
            let result = handler.handle(ListDriveFilesRequest).await;
            writeln!(stdout, "{:?}", result).unwrap();
        } else if line.starts_with("drive_metadata ") {
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let req = GetDriveFileMetadataRequest { file_id: parts[1].to_string() };
                let handler = GetDriveFileMetadataHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: drive_metadata <file_id>").unwrap();
            }
        } else if line.starts_with("permissions_list ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let resource_type = match parts[1] {
                    "spreadsheet" => crate::models::ResourceType::Spreadsheet,
                    "drivefile" => crate::models::ResourceType::DriveFile,
                    _ => {
                        writeln!(stdout, "Unknown resource type").unwrap();
                        continue;
                    }
                };
                let req = ListPermissionsRequest { resource_id: parts[2].to_string(), resource_type };
                let handler = ListPermissionsHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: permissions_list <spreadsheet|drivefile> <resource_id>").unwrap();
            }
        } else if line.starts_with("permissions_add ") {
            let parts: Vec<&str> = line.splitn(6, ' ').collect();
            if parts.len() == 6 {
                let resource_type = match parts[1] {
                    "spreadsheet" => crate::models::ResourceType::Spreadsheet,
                    "drivefile" => crate::models::ResourceType::DriveFile,
                    _ => {
                        writeln!(stdout, "Unknown resource type").unwrap();
                        continue;
                    }
                };
                let req = AddPermissionRequest {
                    resource_id: parts[2].to_string(),
                    resource_type,
                    email: parts[3].to_string(),
                    role: parts[4].to_string(),
                    type_: parts[5].to_string(),
                };
                let handler = AddPermissionHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: permissions_add <spreadsheet|drivefile> <resource_id> <email> <role> <type>").unwrap();
            }
        } else if line.starts_with("permissions_remove ") {
            let parts: Vec<&str> = line.splitn(4, ' ').collect();
            if parts.len() == 4 {
                let resource_type = match parts[1] {
                    "spreadsheet" => crate::models::ResourceType::Spreadsheet,
                    "drivefile" => crate::models::ResourceType::DriveFile,
                    _ => {
                        writeln!(stdout, "Unknown resource type").unwrap();
                        continue;
                    }
                };
                let req = RemovePermissionRequest {
                    resource_id: parts[2].to_string(),
                    resource_type,
                    permission_id: parts[3].to_string(),
                };
                let handler = RemovePermissionHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: permissions_remove <spreadsheet|drivefile> <resource_id> <permission_id>").unwrap();
            }
        } else if line.starts_with("docs_get ") {
            // docs_get <document_id>
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let req = GetDocumentRequest { document_id: parts[1].to_string() };
                let handler = GetDocumentHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: docs_get <document_id>").unwrap();
            }
        } else if line.starts_with("docs_create ") {
            // docs_create <title>
            let parts: Vec<&str> = line.splitn(2, ' ').collect();
            if parts.len() == 2 {
                let req = CreateDocumentRequest { title: parts[1].to_string() };
                let handler = CreateDocumentHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: docs_create <title>").unwrap();
            }
        } else if line.starts_with("docs_append ") {
            // docs_append <document_id> <text>
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let req = AppendTextRequest {
                    document_id: parts[1].to_string(),
                    text: parts[2].to_string(),
                };
                let handler = AppendTextHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: docs_append <document_id> <text>").unwrap();
            }
        } else if line == "docs_list" {
            let handler = crate::models::ListDocsHandler;
            let req = crate::models::ListDocsRequest {};
            let resp = handler.handle(req, &client).await;
            match resp {
                Ok(r) => writeln!(stdout, "{}", serde_json::to_string(&r).unwrap()).unwrap(),
                Err(e) => writeln!(stdout, "error: {}", e).unwrap(),
            }
        // Google Calendar commands
        } else if line.starts_with("calendar_get_event ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let req = GetCalendarEventRequest {
                    event_id: parts[2].to_string(),
                };
                let handler = GetCalendarEventHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: calendar_get_event <event_id>").unwrap();
            }
        } else if line.starts_with("calendar_create_event ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let req = CreateCalendarEventRequest {
                    calendar_id: parts[1].to_string(),
                    event: serde_json::from_str(&parts[2]).unwrap(),
                };
                let handler = CreateCalendarEventHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: calendar_create_event <calendar_id> <event_json>").unwrap();
            }
        } else if line.starts_with("calendar_list_events ") {
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() == 3 {
                let req = ListCalendarEventsRequest {
                    calendar_id: parts[1].to_string(),
                    time_min: parts[2].to_string(),
                };
                let handler = ListCalendarEventsHandler { client: client.clone() };
                let result = handler.handle(req).await;
                writeln!(stdout, "{:?}", result).unwrap();
            } else {
                writeln!(stdout, "Usage: calendar_list_events <calendar_id> <time_min>").unwrap();
            }
        } else {
            writeln!(stdout, "Unknown command").unwrap();
        }
        stdout.flush().unwrap();
    }
}
