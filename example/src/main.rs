use std::collections::HashMap;

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::Value;
use tracing::info;
use wso2_interceptor_types::{
    request_handler_request_body::RequestHandlerRequestBody,
    request_handler_response_body::{DynamicEndpoint, RequestHandlerResponseBody},
    response_handler_request_body::ResponseHandlerRequestBody,
    response_handler_response_body::ResponseHandlerResponseBody,
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/api/v1/handle-request", post(handle_request))
        .route("/api/v1/handle-response", post(handle_response));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(
    Json(payload): Json<RequestHandlerRequestBody>,
) -> Json<RequestHandlerResponseBody> {
    println!("request paylaod body: {:?}", payload.request_body.unwrap());
    let request_handler_response_body = RequestHandlerResponseBody {
        direct_respond: Some(false),
        response_code: Some(200),
        dynamic_endpoint: None,
        headers_to_add: Some({
            let mut headers = HashMap::new();
            headers.insert("foobar".to_string(), "40".to_string());
            headers.insert("baz".to_string(), "40".to_string());
            headers
        }),
        headers_to_replace: None,
        headers_to_remove: None,
        trailers_to_add: Some({
            let mut trailers = HashMap::new();
            trailers.insert("trailer1-key".to_string(), "value".to_string());
            trailers
        }),
        trailers_to_replace: None,
        trailers_to_remove: None,
        body: None,
        interceptor_context: Some({
            let mut context = HashMap::new();
            context.insert("key1".to_string(), "value1".to_string());
            context.insert("key2".to_string(), "value2".to_string());
            context
        }),
    };
    request_handler_response_body.into()
}

async fn handle_response(
    Json(payload): Json<ResponseHandlerRequestBody>,
) -> Json<ResponseHandlerResponseBody> {
    println!("Response payload: {:?}", payload);
    ResponseHandlerResponseBody {
        response_code: Some(200),
        headers_to_add: Some({
            let mut headers = HashMap::new();
            headers.insert("content-type".to_string(), "application/xml".to_string());
            headers.insert("content-length".to_string(), "40".to_string());
            headers
        }),
        headers_to_replace: Some({
            let mut headers = HashMap::new();
            headers.insert("content-type".to_string(), "application/xml".to_string());
            headers.insert("content-length".to_string(), "40".to_string());
            headers
        }),
        headers_to_remove: Some(vec!["key1".to_string(), "key2".to_string()]),
        trailers_to_add: Some({
            let mut trailers = HashMap::new();
            trailers.insert("trailer1-key".to_string(), "value".to_string());
            trailers
        }),
        trailers_to_replace: Some({
            let mut trailers = HashMap::new();
            trailers.insert("trailer1-key".to_string(), "value".to_string());
            trailers
        }),
        trailers_to_remove: Some(vec!["key1".to_string(), "key2".to_string()]),
        body: Some("PGhlbGxvPndvcmxkPC9oZWxsbz4K".to_string()),
    }
    .into()
}
