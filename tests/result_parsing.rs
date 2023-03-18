use moonsock::{MoonMSG, MoonResultData};
#[test]
fn result_ok() {
    let message_struct = MoonMSG::MoonResult {
        jsonrpc: moonsock::JsonRpcVersion::V2_0,
        result: MoonResultData::Ok,
        id: 1340,
    };
    let message_string = serde_json::to_string(&message_struct).unwrap();
    println!("{}", message_string);
    let message = r##"{
        "jsonrpc": "2.0", 
        "result": "ok", 
        "id": 1340
    }"##;
    let message_parsed: MoonMSG = serde_json::from_str(message).unwrap();
    match message_parsed {
        MoonMSG::MoonResult { result, id, .. } => {
            assert_eq!(result, MoonResultData::Ok);
            assert_eq!(id, 1340);
        }
        _ => panic!("Wrong message type"),
    }
}  