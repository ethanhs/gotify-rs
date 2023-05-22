use httpmock::prelude::*;

use gotify_rs::*;

#[test]
fn test_application_apis_sync() {
    let server = MockServer::start();

    let list_app_mock = server.mock(|when, then| {
        when.method(GET).path("/application");
        then.status(200)
            .header("content-type", "application/json")
            .body(
                r#"[
                {
                  "description": "Backup server for the interwebs",
                  "id": 5,
                  "image": "image/image.jpeg",
                  "internal": false,
                  "name": "Backup Server",
                  "token": "AWH0wZ5r0Mbac.r"
                }
              ]"#,
            );
    });

    let url = server.url("/");
    let gotify = SyncGotify::new(&url, Some("fake_app_token"), Some("fake_client_token"));
    let apps = gotify.applications().unwrap();
    assert_eq!(apps.len(), 1);
    assert_eq!(apps[0].id, 5);
    assert_eq!(list_app_mock.hits(), 1);

    let app_name = "app_name".to_string();
    let description = "An application".to_string();
    let create_app_mock = server.mock(|when, then| {
        when.method(POST).path("/application");
        then.status(200)
            .header("content-type", "application/json")
            .body(format!(
                r#"
           {{
            "description": "{}",
            "id": 1,
            "image": "",
            "internal": false,
            "name": "{}",
            "token": "AWH0wZ5r0Mbac.r"
          }}
           "#,
                &description, &app_name
            ));
    });
    let new_app = gotify
        .create_application(app_name.clone(), description.clone())
        .unwrap();
    assert_eq!(new_app.name, app_name);
    assert_eq!(new_app.description, description);
    assert_eq!(create_app_mock.hits(), 1);
    assert_eq!(list_app_mock.hits(), 1);

    let new_app_name = "new_name".to_string();
    let new_description = "Updated application description".to_string();
    let update_app_mock = server.mock(|when, then| {
        when.method(PUT).path("/application/1");
        then.status(200)
            .header("content-type", "application/json")
            .body(format!(
                r#"
           {{
            "description": "{}",
            "id": 1,
            "image": "",
            "internal": false,
            "name": "{}",
            "token": "AWH0wZ5r0Mbac.r"
          }}
           "#,
                &new_description, &new_app_name
            ));
    });
    let new_app = gotify
        .update_application(1, new_app_name.clone(), Some(new_description.clone()))
        .unwrap();
    assert_eq!(new_app.name, new_app_name);
    assert_eq!(new_app.description, new_description);
    assert_eq!(update_app_mock.hits(), 1);
    assert_eq!(create_app_mock.hits(), 1);
    assert_eq!(list_app_mock.hits(), 1);
}
