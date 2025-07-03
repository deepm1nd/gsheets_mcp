use crate::models::{
    AddPermissionRequest, AddPermissionResponse, ListPermissionsRequest, ListPermissionsResponse,
    PermissionInfo, RemovePermissionRequest, RemovePermissionResponse, ResourceType, SpreadsheetError,
};
use async_trait::async_trait;
use std::sync::Arc;

use crate::google_api_client::GoogleApiClient;
use crate::handlers::McpHandler;

pub struct ListPermissionsHandler {
    pub client: Arc<GoogleApiClient>,
}

#[async_trait]
impl McpHandler for ListPermissionsHandler {
    type Request = ListPermissionsRequest;
    type Response = ListPermissionsResponse;
    type Error = SpreadsheetError;
    async fn handle(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        // Call GoogleApiClient with resource_id and resource_type
        // (Stub: returns dummy permissions)
        let permissions = vec![
            PermissionInfo {
                id: "perm1".to_string(),
                email: Some("user1@example.com".to_string()),
                role: "writer".to_string(),
                type_: "user".to_string(),
            },
            PermissionInfo {
                id: "perm2".to_string(),
                email: Some("user2@example.com".to_string()),
                role: "reader".to_string(),
                type_: "user".to_string(),
            },
        ];
        Ok(ListPermissionsResponse { permissions })
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
        // Call GoogleApiClient to add permission (stub)
        Ok(AddPermissionResponse { permission_id: "perm_new".to_string() })
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
        // Call GoogleApiClient to remove permission (stub)
        Ok(RemovePermissionResponse { success: true })
    }
}