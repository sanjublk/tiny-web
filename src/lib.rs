use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub method: String,
    pub headers: HashMap<String, String>,
    pub path: String,
    pub queries: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub body: String,
    pub status_code: i32,
    pub content_type: String,
}
