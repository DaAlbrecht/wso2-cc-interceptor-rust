use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestHandlerRequestBody {
    request_headers: Option<HashMap<String, String>>,
    request_trailers: Option<HashMap<String, String>>,
    request_body: Option<String>,
    invocation_context: Option<InvocationContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InvocationContext {
    request_id: Option<String>,
    protocol: Option<String>,
    scheme: Option<String>,
    api_name: Option<String>,
    api_version: Option<String>,
    vhost: Option<String>,
    supported_methods: Option<String>,
    method: Option<String>,
    base_path: Option<String>,
    path: Option<String>,
    path_template: Option<String>,
    source: Option<String>,
    prod_cluster_name: Option<String>,
    sand_cluster_name: Option<String>,
    authentication_context: Option<AuthenticationContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticationContext {
    token_type: Option<String>,
    token: Option<String>,
    key_type: Option<String>,
}
