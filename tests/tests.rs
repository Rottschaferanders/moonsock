#[test]
fn ok_message() {
    //use crate::json_rpc::MoonMSG;
    use moonsock::MoonMSG;
    let raw = "ok";
    let serialize_raw: MoonMSG = serde_json::from_str(raw).expect("Serialize Failed");
    let deserialize_raw = serde_json::to_string(&serialize_raw).unwrap();
    assert_eq!(raw, deserialize_raw);
    let object = MoonMSG::AvailablePrinterObjects { objects: Vec::from(["gcode".to_string(), "toolhead".to_string(), "bed_mesh".to_string(), "configfile".to_string()]) };
    let deserialize_object = serde_json::to_string(&object).unwrap();
    let serialize_object: MoonMSG = serde_json::from_str(&deserialize_object).unwrap();
    assert_eq!(object, serialize_object);
}

#[test]
fn identify_connection() {
    //use crate::json_rpc::MoonMSG;
    use moonsock::MoonMSG;
    let message = "{\"jsonrpc\":\"2.0\",\"method\":\"server.connection.identify\",\"params\":{\"client_name\":\"moontest\",\"version\":\"0.0.1\",\"type\":\"web\",\"url\":\"http://github.com/arksine/moontest\"},\"id\":4656}";
    let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
    let deserialized = serde_json::to_string(&serialized).unwrap();
    println!("{:?}", &deserialized);
    assert_eq!(message, deserialized);
}

#[test]
fn printer_restart() {
    //use crate::json_rpc::MoonMSG;
    use moonsock::MoonMSG;
    let message = "{\"jsonrpc\":\"2.0\",\"method\":\"printer.restart\",\"id\":4894}";
    let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
    let deserialized = serde_json::to_string(&serialized).unwrap();
    println!("{:?}", &deserialized);
    assert_eq!(message, deserialized);
}

#[test]
fn query_endstops() {
    //use crate::json_rpc::MoonMSG;
    use moonsock::MoonMSG;
    let message = "{\"jsonrpc\":\"2.0\",\"method\":\"printer.query_endstops.status\",\"id\":3456}";
    let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
    let deserialized = serde_json::to_string(&serialized).unwrap();
    println!("{:?}", &deserialized);
    assert_eq!(message, deserialized);
}

#[test]
fn notify_proc_stat_update() {
    //use crate::json_rpc::MoonMSG;
    use moonsock::MoonMSG;
    let message = "{\"jsonrpc\":\"2.0\",\"method\":\"notify_proc_stat_update\",\"params\":{\"moonraker_stats\":{\"time\":1656815808.6346357,\"cpu_usage\":1.9,\"memory\":33008,\"mem_units\":\"kB\"},\"cpu_temp\":35.05,\"network\":{\"lo\":{\"rx_bytes\":1570,\"tx_bytes\":1570,\"bandwidth\":0.0},\"eth0\":{\"rx_bytes\":0,\"tx_bytes\":0,\"bandwidth\":0.0},\"wlan0\":{\"rx_bytes\":2101316,\"tx_bytes\":1788446,\"bandwidth\":961.51},\"docker0\":{\"rx_bytes\":0,\"tx_bytes\":0,\"bandwidth\":0.0}},\"system_cpu_usage\":{\"cpu\":0.75,\"cpu0\":1.0,\"cpu1\":1.98,\"cpu2\":1.0,\"cpu3\":1.98},\"system_memory\":{\"total\":3705692,\"available\":3408144,\"used\":297548},\"websocket_connections\":1}}";
    let serialized: MoonMSG = serde_json::from_str(&message).unwrap();
    let deserialized = serde_json::to_string(&serialized).unwrap();
    println!("{:?}", &deserialized);
    assert_eq!(message, deserialized);
}

#[test]
fn printer_objects() {
    //use crate::json_rpc::MoonMSG;
    use moonsock::MoonMSG;
    let raw = "{\"objects\":[\"gcode\",\"toolhead\",\"bed_mesh\",\"configfile\"]}";
    let serialize_raw: MoonMSG = serde_json::from_str(raw).expect("Serialize Failed");
    let deserialize_raw = serde_json::to_string(&serialize_raw).unwrap();
    assert_eq!(raw, deserialize_raw);
    let object = MoonMSG::AvailablePrinterObjects { objects: Vec::from(["gcode".to_string(), "toolhead".to_string(), "bed_mesh".to_string(), "configfile".to_string()]) };
    let deserialize_object = serde_json::to_string(&object).unwrap();
    let serialize_object: MoonMSG = serde_json::from_str(&deserialize_object).unwrap();
    assert_eq!(object, serialize_object);
}