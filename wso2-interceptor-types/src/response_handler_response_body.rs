use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseHandlerResponseBody {
    pub response_code: Option<u16>,
    pub headers_to_add: Option<HashMap<String, String>>,
    pub headers_to_replace: Option<HashMap<String, String>>,
    pub headers_to_remove: Option<Vec<String>>,
    pub trailers_to_add: Option<HashMap<String, String>>,
    pub trailers_to_replace: Option<HashMap<String, String>>,
    pub trailers_to_remove: Option<Vec<String>>,
    pub body: Option<String>,
}
