use moonsock::{MoonResponse, JsonRpcVersion, NotificationMethod};

#[test]
fn test_deserialize_notify_klippy_shutdown() {
    let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_klippy_shutdown"
    }"#;

    let expected = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2_0,
        method: NotificationMethod::NotifyKlippyShutdown,
        params: None,
    };

    let actual: MoonResponse = serde_json::from_str(json).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_serialize_notify_klippy_shutdown() {
    let data = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2_0,
        method: NotificationMethod::NotifyKlippyShutdown,
        params: None,
    };

    let expected = r#"{"jsonrpc":"2.0","method":"notify_klippy_shutdown"}"#;
    let actual = serde_json::to_string(&data).unwrap();
    assert_eq!(actual, expected);
}
