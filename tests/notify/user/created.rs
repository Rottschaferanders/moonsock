use moonsock::{
    MoonResponse, JsonRpcVersion,
    NotificationMethod, NotificationParam,
    UserParam,
};

#[test]
fn test_deserialize_notify_user_created() {
    let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_user_created",
        "params": [
            {
                "username": "testuser"
            }
        ]
    }"#;

    let expected = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyUserCreated,
        params: Some(NotificationParam::User(vec![
            UserParam {
                username: "testuser".to_string(),
            },
        ])),
    };

    let actual: MoonResponse = serde_json::from_str(json).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_serialize_notify_user_created() {
    let data = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyUserCreated,
        params: Some(NotificationParam::User(vec![
            UserParam {
                username: "testuser".to_string(),
            },
        ])),
    };

    let expected = r#"{"jsonrpc":"2.0","method":"notify_user_created","params":[{"username":"testuser"}]}"#;
    let actual = serde_json::to_string(&data).unwrap();
    assert_eq!(actual, expected);
}