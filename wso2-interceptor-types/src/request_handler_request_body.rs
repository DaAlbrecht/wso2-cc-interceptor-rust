use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestHandlerRequestBody {
    pub request_headers: Option<HashMap<String, String>>,
    pub request_trailers: Option<HashMap<String, String>>,
    pub request_body: Option<String>,
    pub invocation_context: Option<InvocationContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvocationContext {
    pub request_id: Option<String>,
    pub protocol: Option<String>,
    pub scheme: Option<String>,
    pub api_name: Option<String>,
    pub api_version: Option<String>,
    pub vhost: Option<String>,
    pub supported_methods: Option<String>,
    pub method: Option<String>,
    pub base_path: Option<String>,
    pub path: Option<String>,
    pub path_template: Option<String>,
    pub source: Option<String>,
    pub prod_cluster_name: Option<String>,
    pub sand_cluster_name: Option<String>,
    pub authentication_context: Option<AuthenticationContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationContext {
    pub token_type: Option<String>,
    pub token: Option<String>,
    pub key_type: Option<String>,
}
