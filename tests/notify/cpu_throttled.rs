use moonsock::{MoonResponse, JsonRpcVersion, NotificationMethod, NotificationParam, CpuThrottledState};

#[test]
fn test_parse_notify_cpu_throttled() {
    let json = r#"{
        "jsonrpc": "2.0",
        "method": "notify_cpu_throttled",
        "params": [
            {
                "bits": 0,
                "flags": []
            }
        ]
    }"#;

    let expected = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyCpuThrottled,
        params: Some(
            NotificationParam::CpuThrottled(vec![CpuThrottledState {
                bits: 0,
                flags: vec![],
            }]),
        ),
    };

    let actual: MoonResponse = serde_json::from_str(json).unwrap();
    assert_eq!(actual, expected);

    let serialized = serde_json::to_string(&actual).unwrap();
    let deserialized: MoonResponse = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, expected);
}