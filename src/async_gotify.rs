use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;
use reqwest::Method;
use serde::de::DeserializeOwned;

use async_trait::async_trait;
use reqwest::Client as AsyncClient;

use crate::response_types::*;
use crate::Gotify;

pub struct AsyncGotify<'a> {
    gotify: Gotify<'a>,
    client: AsyncClient,
}

impl<'a> AsyncGotify<'a> {
    pub fn new(
        base_url: &'a str,
        app_token: Option<&'a str>,
        client_token: Option<&'a str>,
    ) -> Self {
        let gotify = Gotify::new(base_url, app_token, client_token);
        let client = AsyncClient::new();
        Self { gotify, client }
    }
    pub fn from(gotify: Gotify<'a>) -> Self {
        let client = AsyncClient::new();
        Self { gotify, client }
    }

    fn get_token(&self, auth_mode: Option<&str>) -> Option<&'a str> {
        if let Some(mode) = auth_mode {
            if mode == "app" {
                self.gotify.app_token
            } else {
                self.gotify.client_token
            }
        } else {
            self.gotify.app_token
        }
    }
}

#[async_trait]
trait AsyncGotifyImpl {
    async fn do_request<T: DeserializeOwned>(
        &self,
        method: &str,
        endpoint_url: &str,
        data: Option<HashMap<String, Option<String>>>,
        file: Option<tokio::fs::File>,
        auth_mode: Option<&str>,
    ) -> Result<T>;

    async fn applications(&self) -> Result<Vec<Application>>;

    async fn create_application(&self, name: String, description: String) -> Result<Application>;

    async fn update_application(
        &self,
        id: i32,
        name: String,
        description: Option<String>,
    ) -> Result<Application>;

    async fn delete_application(&self, id: i32) -> Result<()>;

    async fn upload_application_image(
        &self,
        id: i32,
        image: tokio::fs::File,
    ) -> Result<Application>;

    async fn get_messages(
        &self,
        app_id: Option<i32>,
        limit: Option<i32>,
        since: Option<i32>,
    ) -> Result<PagedMessages>;

    async fn create_message(
        &self,
        message: String,
        priority: Option<i32>,
        title: Option<String>,
        // TODO: extras
    ) -> Result<Message>;

    async fn delete_messages(&self, app_id: Option<i32>) -> Result<()>;

    async fn delete_message(&self, msg_id: i32) -> Result<()>;

    async fn get_clients(&self) -> Result<Vec<Client>>;

    async fn create_client(&self, name: String) -> Result<Client>;

    async fn update_client(&self, id: i32, name: String) -> Result<Client>;

    async fn delete_client(&self, id: i32) -> Result<()>;

    async fn get_current_user(&self) -> Result<User>;

    async fn set_password(&self, passwd: String) -> Result<()>;

    async fn get_users(&self) -> Result<Vec<User>>;

    async fn create_user(&self, name: String, passwd: String, admin: Option<bool>) -> Result<User>;

    async fn get_user(&self, id: i32) -> Result<User>;

    async fn update_user(
        &self,
        id: i32,
        name: Option<String>,
        passwd: Option<String>,
        admin: Option<bool>,
    ) -> Result<User>;

    async fn delete_user(&self, id: i32) -> Result<()>;

    async fn get_health(&self) -> Result<Health>;

    async fn get_plugins(&self) -> Result<Vec<PluginConf>>;

    async fn get_plugin_config(&self, id: i32) -> Result<PluginConf>;

    /// TODO(ethanhs): Figure out what this looks like
    /* async fn update_plugin_config(&self, id: i32); */

    async fn disable_plugin(&self, id: i32) -> Result<()>;

    async fn get_plugin_display(&self, id: i32) -> Result<String>;

    async fn enable_plugin(&self, id: i32) -> Result<()>;

    async fn get_version(&self) -> Result<VersionInfo>;
}

#[async_trait]
impl<'a> AsyncGotifyImpl for AsyncGotify<'a> {
    async fn do_request<T: DeserializeOwned>(
        &self,
        method: &str,
        endpoint_url: &str,
        data: Option<HashMap<String, Option<String>>>,
        file: Option<tokio::fs::File>,
        auth_mode: Option<&str>,
    ) -> Result<T> {
        let method = Method::from_str(&method)?;
        let request_url = format!("{}/{}", self.gotify.base_url, endpoint_url);
        let mut request = self.client.request(method.clone(), request_url);
        if let Some(f) = file {
            request = request.body(f);
        } else {
            if let Some(data) = data {
                match method {
                    Method::GET => {
                        request = request.query(&data);
                    }
                    _ => {
                        request = request.json(&data);
                    }
                }
                request = request.json(&data);
            }
        }
        let token = self.get_token(auth_mode).expect("missing token");
        request = request.header("X-Gotify-Key", token);
        let response = request.send().await?;
        Ok(response.json::<T>().await?)
    }

    async fn applications(&self) -> Result<Vec<Application>> {
        self.do_request("get", "/applications", None, None, None)
            .await
    }

    async fn create_application(&self, name: String, description: String) -> Result<Application> {
        let mut data = HashMap::new();
        data.insert("name".to_owned(), Some(name));
        data.insert("description".to_owned(), Some(description));
        self.do_request("post", "/application", Some(data), None, None)
            .await
    }

    async fn update_application(
        &self,
        id: i32,
        name: String,
        description: Option<String>,
    ) -> Result<Application> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Some(name));
        data.insert("description".to_string(), description);
        self.do_request("put", &format!("/application/{id}"), Some(data), None, None)
            .await
    }

    async fn delete_application(&self, id: i32) -> Result<()> {
        self.do_request("delete", &format!("/application/{id}"), None, None, None)
            .await
    }

    async fn upload_application_image(
        &self,
        id: i32,
        image: tokio::fs::File,
    ) -> Result<Application> {
        self.do_request(
            "post",
            &format!("/application/{id}/image"),
            None,
            Some(image),
            None,
        )
        .await
    }

    async fn get_messages(
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
            .await
        } else {
            self.do_request("get", "/message", Some(data), None, None)
                .await
        }
    }

    async fn create_message(
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
            .await
    }

    async fn delete_messages(&self, app_id: Option<i32>) -> Result<()> {
        if let Some(id) = app_id {
            self.do_request(
                "delete",
                &format!("/application/{id}/message"),
                None,
                None,
                None,
            )
            .await
        } else {
            self.do_request("delete", &format!("/message"), None, None, None)
                .await
        }
    }

    async fn delete_message(&self, msg_id: i32) -> Result<()> {
        self.do_request("delete", &format!("/message/{msg_id}"), None, None, None)
            .await
    }

    async fn get_clients(&self) -> Result<Vec<Client>> {
        self.do_request("get", "/client", None, None, None).await
    }

    async fn create_client(&self, name: String) -> Result<Client> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Some(name));
        self.do_request("post", "/client", Some(data), None, None)
            .await
    }

    async fn update_client(&self, id: i32, name: String) -> Result<Client> {
        let mut data = HashMap::new();
        data.insert("name".to_string(), Some(name));
        self.do_request("put", &format!("/client/{id}"), Some(data), None, None)
            .await
    }

    async fn delete_client(&self, id: i32) -> Result<()> {
        self.do_request("delete", &format!("/client/{id}"), None, None, None)
            .await
    }

    async fn get_current_user(&self) -> Result<User> {
        self.do_request("get", "/current/user", None, None, None)
            .await
    }

    async fn set_password(&self, passwd: String) -> Result<()> {
        let mut data = HashMap::new();
        data.insert("pass".to_string(), Some(passwd));
        self.do_request("get", "/current/user/password", Some(data), None, None)
            .await
    }

    async fn get_users(&self) -> Result<Vec<User>> {
        self.do_request("get", "/user", None, None, None).await
    }

    async fn create_user(&self, name: String, passwd: String, admin: Option<bool>) -> Result<User> {
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
            .await
    }

    async fn get_user(&self, id: i32) -> Result<User> {
        self.do_request("get", &format!("/user/{id}"), None, None, None)
            .await
    }

    async fn update_user(
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
            .await
    }

    async fn delete_user(&self, id: i32) -> Result<()> {
        self.do_request("delete", &format!("/user/{id}"), None, None, None)
            .await
    }

    async fn get_health(&self) -> Result<Health> {
        self.do_request("get", "/health", None, None, None).await
    }

    async fn get_plugins(&self) -> Result<Vec<PluginConf>> {
        self.do_request("get", "/plugins", None, None, None).await
    }

    async fn get_plugin_config(&self, id: i32) -> Result<PluginConf> {
        self.do_request("get", &format!("/plugins/{id}/config"), None, None, None)
            .await
    }

    /// TODO(ethanhs): Figure out what this looks like
    /* async fn update_plugin_config(&self, id: i32) {
        unimplemented!()
    } */

    async fn disable_plugin(&self, id: i32) -> Result<()> {
        self.do_request("post", &format!("/plugins/{id}/disable"), None, None, None)
            .await
    }

    async fn get_plugin_display(&self, id: i32) -> Result<String> {
        self.do_request("get", &format!("/plugins/{id}/display"), None, None, None)
            .await
    }

    async fn enable_plugin(&self, id: i32) -> Result<()> {
        self.do_request("post", &format!("/plugins/{id}/enable"), None, None, None)
            .await
    }

    async fn get_version(&self) -> Result<VersionInfo> {
        self.do_request("get", "/version", None, None, None).await
    }
}
