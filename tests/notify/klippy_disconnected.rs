use moonsock::{MoonResponse, JsonRpcVersion, NotificationMethod};

#[test]
fn test_deserialize_notify_klippy_disconnected() {
    let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_klippy_disconnected"
    }"#;

    let expected = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyKlippyDisconnected,
        params: None,
    };

    let actual: MoonResponse = serde_json::from_str(json).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_serialize_notify_klippy_disconnected() {
    let data = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyKlippyDisconnected,
        params: None,
    };

    let expected = r#"{"jsonrpc":"2.0","method":"notify_klippy_disconnected"}"#;
    let actual = serde_json::to_string(&data).unwrap();
    assert_eq!(actual, expected);
}