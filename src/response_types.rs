use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Application {
    description: String,
    id: i32,
    image: String,
    internal: bool,
    name: String,
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    id: i32,
    name: String,
    token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    error: String,
    #[serde(rename = "errorCode")]
    code: i32,
    #[serde(rename = "errorDescription")]
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Health {
    database: String,
    health: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    appid: i32,
    date: String,
    extras: HashMap<String, serde_json::Value>,
    id: i32,
    message: String,
    priority: i32,
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PagedMessages {
    messages: Vec<Message>,
    paging: Paging,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paging {
    limit: i32,
    next: String,
    since: i32,
    size: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginConf {
    author: String,
    capabilities: Vec<String>,
    enabled: bool,
    id: i32,
    license: String,
    #[serde(rename = "modulePath")]
    module_path: String,
    name: String,
    token: String,
    website: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    admin: bool,
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPass {
    pass: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWithPass {
    admin: bool,
    id: i32,
    name: String,
    pass: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionInfo {
    #[serde(rename = "buildDate")]
    build_date: String,
    commit: String,
    version: String,
}
