use moonsock::{
    MoonResponse, JsonRpcVersion,
    NotificationMethod, NotificationParam,
    UserParam,
};

#[test]
fn test_deserialize_notify_user_deleted() {
    let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_user_deleted",
        "params": [
            {
                "username": "testuser"
            }
        ]
    }"#;

    let expected = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2_0,
        method: NotificationMethod::NotifyUserDeleted,
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
fn test_serialize_notify_user_deleted() {
    let data = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2_0,
        method: NotificationMethod::NotifyUserDeleted,
        params: Some(NotificationParam::User(vec![
            UserParam {
                username: "testuser".to_string(),
            },
        ])),
    };

    let expected = r#"{"jsonrpc":"2.0","method":"notify_user_deleted","params":[{"username":"testuser"}]}"#;
    let actual = serde_json::to_string(&data).unwrap();
    assert_eq!(actual, expected);
}