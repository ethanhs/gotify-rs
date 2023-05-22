use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Application {
    pub description: String,
    pub id: i32,
    pub image: String,
    pub internal: bool,
    pub name: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub id: i32,
    pub name: String,
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub error: String,
    #[serde(rename = "errorCode")]
    pub code: i32,
    #[serde(rename = "errorDescription")]
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Health {
    pub database: String,
    pub health: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub appid: i32,
    pub date: String,
    pub extras: HashMap<String, serde_json::Value>,
    pub id: i32,
    pub message: String,
    pub priority: i32,
    pub title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PagedMessages {
    pub messages: Vec<Message>,
    pub paging: Paging,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Paging {
    pub limit: i32,
    pub next: String,
    pub since: i32,
    pub size: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PluginConf {
    pub author: String,
    pub capabilities: Vec<String>,
    pub enabled: bool,
    pub id: i32,
    pub license: String,
    #[serde(rename = "modulePath")]
    pub module_path: String,
    pub name: String,
    pub token: String,
    pub website: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub admin: bool,
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserPass {
    pub pass: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserWithPass {
    pub admin: bool,
    pub id: i32,
    pub name: String,
    pub pass: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VersionInfo {
    #[serde(rename = "buildDate")]
    pub build_date: String,
    pub commit: String,
    pub version: String,
}
