use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;

use reqwest::blocking::Client as SyncClient;
use reqwest::Method;

use anyhow::Result;
use serde::de::DeserializeOwned;

mod response_types;

pub use self::response_types::*;

pub struct Gotify<'a> {
    base_url: &'a str,
    app_token: &'a str,
    client_token: &'a str,
}

impl<'a> Gotify<'a> {
    pub fn new(base_url: &'a str, app_token: &'a str, client_token: &'a str) -> Self {
        Self {
            base_url: base_url.trim_end_matches("/"),
            app_token,
            client_token,
        }
    }
    pub fn config(mut self, base_url: &'a str, app_token: &'a str, client_token: &'a str) -> Self {
        self.base_url = base_url.trim_end_matches("/");
        self.app_token = app_token;
        self.client_token = client_token;
        self
    }
}

pub struct SyncGotify<'a> {
    gotify: Gotify<'a>,
    client: SyncClient,
}

impl<'a> SyncGotify<'a> {
    pub fn new(base_url: &'a str, app_token: &'a str, client_token: &'a str) -> Self {
        let gotify = Gotify::new(base_url, app_token, client_token);
        let client = SyncClient::new();
        Self { gotify, client }
    }
    pub fn from(gotify: Gotify<'a>) -> Self {
        let client = SyncClient::new();
        Self { gotify, client }
    }

    fn do_request<T: DeserializeOwned>(
        &self,
        method: &str,
        endpoint_url: &str,
        data: Option<HashMap<String, Option<String>>>,
        file: Option<File>,
        auth_mode: Option<&str>,
    ) -> Result<T> {
        let request_url = format!("{}/{}", self.gotify.base_url, endpoint_url);
        if let Some(f) = file {
            let response = self
                .client
                .request(Method::from_str(&method)?, request_url)
                .body(f)
                .send()?;
            Ok(response.json::<T>()?)
        } else {
            let mut request = self.client.request(Method::from_str(&method)?, request_url);
            if let Some(data) = data {
                request = request.json(&data);
            }
            let response = request.send()?;
            Ok(response.json::<T>()?)
        }
    }

    pub fn applications(&self) -> Result<Vec<Application>> {
        self.do_request("get", "/applications", None, None, None)
    }

    pub fn create_application(&self, name: String, description: String) -> Result<Application> {
        let mut data = HashMap::new();
        data.insert("name".to_owned(), Some(name));
        data.insert("description".to_owned(), Some(description));
        self.do_request("post", "/application", Some(data), None, None)
    }

    pub fn update_application(
        &self,
        id: i32,
        name: String,
        description: Option<String>,
    ) -> Result<Application> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Some(name));
        data.insert("description".to_string(), description);
        self.do_request("put", &format!("/application/{id}"), Some(data), None, None)
    }

    pub fn delete_application(&self, id: i32) -> Result<()> {
        self.do_request("delete", &format!("/application/{id}"), None, None, None)
    }

    pub fn upload_application_image(&self, id: i32, image: File) -> Result<Application> {
        self.do_request(
            "post",
            &format!("/application/{id}/image"),
            None,
            Some(image),
            None,
        )
    }

    pub fn get_messages(
        &self,
        app_id: Option<i32>,
        limit: Option<i32>,
        since: Option<i32>,
    ) -> Result<PagedMessages> {
        let mut data = HashMap::new();
        data.insert("limit".to_string(), limit.map(|i| i.to_string()));
        data.insert("since".to_string(), since.map(|i| i.to_string()));
        if let Some(id) = app_id {
            self.do_request(
                "get",
                &format!("/application/{id}/message"),
                Some(data),
                None,
                None,
            )
        } else {
            self.do_request("get", "/message", Some(data), None, None)
        }
    }

    pub fn create_message(
        &self,
        message: String,
        priority: Option<i32>,
        title: Option<String>,
        // TODO: extras
    ) -> Result<Message> {
        let mut data = HashMap::new();
        data.insert("message".to_string(), Some(message));
        data.insert("priority".to_string(), priority.map(|i| i.to_string()));
        data.insert("title".to_string(), title);
        self.do_request("post", "/message", Some(data), None, Some("app"))
    }

    pub fn delete_messages(&self, app_id: Option<i32>) -> Result<()> {
        if let Some(id) = app_id {
            self.do_request(
                "delete",
                &format!("/application/{id}/message"),
                None,
                None,
                None,
            )
        } else {
            self.do_request("delete", &format!("/message"), None, None, None)
        }
    }

    pub fn delete_message(&self, msg_id: i32) -> Result<()> {
        self.do_request("delete", &format!("/message/{msg_id}"), None, None, None)
    }

    pub fn get_clients(&self) -> Result<Vec<Client>> {
        self.do_request("get", "/client", None, None, None)
    }

    pub fn create_client(&self, name: String) -> Result<Client> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Some(name));
        self.do_request("post", "/client", Some(data), None, None)
    }

    pub fn update_client(&self, id: i32, name: String) -> Result<Client> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Some(name));
        self.do_request("put", &format!("/client/{id}"), Some(data), None, None)
    }

    pub fn delete_client(&self, id: i32) -> Result<()> {
        self.do_request("delete", &format!("/client/{id}"), None, None, None)
    }

    pub fn get_current_user(&self) -> Result<User> {
        self.do_request("get", "/current/user", None, None, None)
    }

    pub fn set_password(&self, passwd: String) -> Result<()> {
        let mut data = HashMap::new();
        data.insert("pass".to_string(), Some(passwd));
        self.do_request("get", "/current/user/password", Some(data), None, None)
    }

    pub fn get_users(&self) -> Result<Vec<User>> {
        self.do_request("get", "/user", None, None, None)
    }

    pub fn create_user(&self, name: String, passwd: String, admin: Option<bool>) -> Result<User> {
        let mut data = HashMap::new();
        data.insert("pass".to_string(), Some(passwd));
        data.insert("name".to_string(), Some(name));
        data.insert(
            "admin".to_string(),
            Some(if admin.unwrap_or(false) {
                "true".to_string()
            } else {
                "false".to_string()
            }),
        );
        self.do_request("post", "/user", Some(data), None, None)
    }

    pub fn get_user(&self, id: i32) -> Result<User> {
        self.do_request("get", &format!("/user/{id}"), None, None, None)
    }

    pub fn update_user(
        &self,
        id: i32,
        name: Option<String>,
        passwd: Option<String>,
        admin: Option<bool>,
    ) -> Result<User> {
        let mut data = HashMap::new();
        data.insert("pass".to_string(), passwd);
        data.insert("name".to_string(), name);
        data.insert(
            "admin".to_string(),
            Some(if admin.unwrap_or(false) {
                "true".to_string()
            } else {
                "false".to_string()
            }),
        );
        self.do_request("put", &format!("/user/{id}"), Some(data), None, None)
    }

    pub fn delete_user(&self, id: i32) -> Result<()> {
        self.do_request("delete", &format!("/user/{id}"), None, None, None)
    }

    pub fn get_health(&self) -> Result<Health> {
        self.do_request("get", "/health", None, None, None)
    }

    pub fn get_plugins(&self) -> Result<Vec<PluginConf>> {
        self.do_request("get", "/plugins", None, None, None)
    }

    pub fn get_plugin_config(&self, id: i32) -> Result<PluginConf> {
        self.do_request("get", &format!("/plugins/{id}/config"), None, None, None)
    }

    /// TODO(ethanhs): Figure out what this looks like
    /* pub fn update_plugin_config(&self, id: i32) {
        unimplemented!()
    } */

    pub fn disable_plugin(&self, id: i32) -> Result<()> {
        self.do_request("post", &format!("/plugins/{id}/disable"), None, None, None)
    }

    pub fn get_plugin_display(&self, id: i32) -> Result<String> {
        self.do_request("get", &format!("/plugins/{id}/display"), None, None, None)
    }

    pub fn enable_plugin(&self, id: i32) -> Result<()> {
        self.do_request("post", &format!("/plugins/{id}/enable"), None, None, None)
    }

    pub fn get_version(&self) -> Result<VersionInfo> {
        self.do_request("get", "/version", None, None, None)
    }
}
