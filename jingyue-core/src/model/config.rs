use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Clone)]
pub struct GetConfigParams {
    pub namespace_id: Option<String>,
    pub group_name: String,
    pub data_id: String,
}


#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigResponse {
    pub result_code: i32,
    pub error_code: i32,
    pub message: Option<String>,
    pub request_id: Option<String>,
    pub content: String,
    pub encrypted_data_key: Option<String>,
    pub content_type: String,
    pub md5: String,
    pub last_modified: i64,
    pub tag: Option<String>,
    pub beta: bool,
    pub success: bool,
}

impl From<HashMap<String, String>> for GetConfigParams {
    fn from(value: HashMap<String, String>) -> Self {
        GetConfigParams {
            namespace_id: value
                .get("namespaceId")
                .map(|s| Some(s.to_string()))
                .unwrap_or_default(),
            group_name: value.get("groupName").unwrap().to_string(),
            data_id: value.get("dataId").unwrap().to_string(),
        }
    }
}
