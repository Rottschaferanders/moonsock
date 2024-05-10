use serde_json::json;
use moonsock::{MoonResponse, NotificationMethod};

#[test]
fn test_parse_notification_klippy_ready() {
    let json = json!({
        "jsonrpc": "2.0",
        "method": "notify_klippy_ready"
    });
    let notification: MoonResponse = serde_json::from_value(json).unwrap();
    match notification {
        MoonResponse::Notification { method, params, .. } => {
            assert_eq!(method, NotificationMethod::NotifyKlippyReady);
            assert_eq!(params, None);
        }
        _ => panic!("Invalid response type"),
    }
}