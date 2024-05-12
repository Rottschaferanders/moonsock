use moonsock::{
    response::EntryId, JsonRpcVersion, MoonResponse, NotificationMethod, NotificationParam
    // response::AnnouncementDismissedParam,
    // response::AnnouncementEntryId,
};

#[test]
fn test_serialize_notify_announcement_dismissed() {
    let message = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyAnnouncementDismissed,
        params: Some(NotificationParam::AnnouncementEntryId(EntryId {
            entry_id: "arksine/moonlight/issue/3".to_string(),
        })),
    };

    let expected_json = r#"{"jsonrpc":"2.0","method":"notify_announcement_dismissed","params":[{"entry_id":"arksine/moonlight/issue/3"}]}"#;
    let actual_json = serde_json::to_string(&message).unwrap();

    assert_eq!(expected_json, actual_json);
}

#[test]
fn test_deserialize_notify_announcement_dismissed() {
    let json = r#"{"jsonrpc":"2.0","method":"notify_announcement_dismissed","params":[{"entry_id":"arksine/moonlight/issue/3"}]}"#;
    let expected_message = MoonResponse::Notification {
        jsonrpc: JsonRpcVersion::V2,
        method: NotificationMethod::NotifyAnnouncementDismissed,
        // params: Some(NotificationParam::AnnouncementDismissed(AnnouncementDismissedParam {
        //     entry_id: "arksine/moonlight/issue/3".to_string(),
        // })),
        params: Some(NotificationParam::AnnouncementEntryId(EntryId {
            entry_id: "arksine/moonlight/issue/3".to_string(),
        })),
    };

    let actual_message: MoonResponse = serde_json::from_str(json).unwrap();

    assert_eq!(expected_message, actual_message);
}
