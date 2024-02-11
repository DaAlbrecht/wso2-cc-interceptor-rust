use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestHandlerResponseBody {
    pub direct_respond: Option<bool>,
    pub response_code: Option<u32>,
    pub dynamic_endpoint: Option<DynamicEndpoint>,
    pub headers_to_add: Option<HashMap<String, String>>,
    pub headers_to_replace: Option<HashMap<String, String>>,
    pub headers_to_remove: Option<Vec<String>>,
    pub trailers_to_add: Option<HashMap<String, String>>,
    pub trailers_to_replace: Option<HashMap<String, String>>,
    pub trailers_to_remove: Option<Vec<String>>,
    pub body: Option<String>,
    pub interceptor_context: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DynamicEndpoint {
    pub endpoint_name: String,
}
