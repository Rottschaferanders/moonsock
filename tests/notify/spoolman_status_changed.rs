use moonsock::{
    MoonResponse, JsonRpcVersion,
    NotificationMethod, NotificationParam,
    response::SpoolmanStatusChangedParams,
};

#[test]
fn test_serialize_notify_spoolman_status_changed() {
    let message = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifySpoolmanStatusChanged,
        params: Some(NotificationParam::SpoolmanStatusChanged(SpoolmanStatusChangedParams {
            spoolman_connected: false,
        })),
    };

    let expected_json = r#"{"jsonrpc":"2.0","method":"notify_spoolman_status_changed","params":[{"spoolman_connected":false}]}"#;
    let actual_json = serde_json::to_string(&message).unwrap();

    assert_eq!(expected_json, actual_json);
}

#[test]
fn test_deserialize_notify_spoolman_status_changed() {
    let json = r#"{"jsonrpc":"2.0","method":"notify_spoolman_status_changed","params":[{"spoolman_connected":false}]}"#;
    let expected_message = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifySpoolmanStatusChanged,
        params: Some(NotificationParam::SpoolmanStatusChanged(SpoolmanStatusChangedParams {
            spoolman_connected: false,
        })),
    };

    let actual_message: MoonResponse = serde_json::from_str(json).unwrap();

    assert_eq!(expected_message, actual_message);
}