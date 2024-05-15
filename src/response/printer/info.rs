// {
//     "jsonrpc": "2.0",
//     "result": {
//         "state": "ready",
//         "state_message": "Printer is ready",
//         "hostname": "my-pi-hostname",
//         "software_version": "v0.9.1-302-g900c7396",
//         "cpu_info": "4 core ARMv7 Processor rev 4 (v7l)",
//         "klipper_path": "/home/pi/klipper",
//         "python_path": "/home/pi/klippy-env/bin/python",
//         "log_file": "/tmp/klippy.log",
//         "config_file": "/home/pi/printer.cfg",
//     },
//     "id": 345
// }


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrinterState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "standby")]
    Standby,
    #[serde(rename = "printing")]
    Printing,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterInfoResponse {
    pub state: PrinterState,
    pub state_message: String,
    pub hostname: String,
    pub software_version: String,
    pub cpu_info: String,
    pub klipper_path: String,
    pub python_path: String,
    // pub process_id: i32,
    // pub user_id: i32,
    // pub group_id: i32,
    pub log_file: String,
    pub config_file: String,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use crate::{MoonResponse, JsonRpcVersion, response::MoonResultData, FastMoonConn};

    #[test]
    fn test_serialize() {
        let printer_info = PrinterInfoResponse {
            state: PrinterState::Ready,
            state_message: "Printer is ready".to_string(),
            hostname: "my-pi-hostname".to_string(),
            software_version: "v0.9.1-302-g900c7396".to_string(),
            cpu_info: "4 core ARMv7 Processor rev 4 (v7l)".to_string(),
            klipper_path: "/home/pi/klipper".to_string(),
            python_path: "/home/pi/klippy-env/bin/python".to_string(),
            log_file: "/tmp/klippy.log".to_string(),
            config_file: "/home/pi/printer.cfg".to_string(),
            // process_id: 0,
            // user_id: 0,
            // group_id: 0,
        };

        let response = MoonResponse::MoonResult {
            jsonrpc: JsonRpcVersion::V2,
            result: MoonResultData::PrinterInfoResponse(printer_info),
            id: 345
        };

        let expected = r#"{"jsonrpc":"2.0","result":{"state":"ready","state_message":"Printer is ready","hostname":"my-pi-hostname","software_version":"v0.9.1-302-g900c7396","cpu_info":"4 core ARMv7 Processor rev 4 (v7l)","klipper_path":"/home/pi/klipper","python_path":"/home/pi/klippy-env/bin/python","log_file":"/tmp/klippy.log","config_file":"/home/pi/printer.cfg"},"id":345}"#;
        assert_eq!(serde_json::to_string(&response).unwrap(), expected);
    }

    #[test]
    fn test_deserialize() {
        let input = r#"{"jsonrpc":"2.0","result":{"state":"ready","state_message":"Printer is ready","hostname":"my-pi-hostname","software_version":"v0.9.1-302-g900c7396","cpu_info":"4 core ARMv7 Processor rev 4 (v7l)","klipper_path":"/home/pi/klipper","python_path":"/home/pi/klippy-env/bin/python","log_file":"/tmp/klippy.log","config_file":"/home/pi/printer.cfg","process_id":0,"user_id":0,"group_id":0},"id":345}"#;
        let response: MoonResponse = serde_json::from_str(input).unwrap();

        let printer_info = PrinterInfoResponse {
            state: PrinterState::Ready,
            state_message: "Printer is ready".to_string(),
            hostname: "my-pi-hostname".to_string(),
            software_version: "v0.9.1-302-g900c7396".to_string(),
            cpu_info: "4 core ARMv7 Processor rev 4 (v7l)".to_string(),
            klipper_path: "/home/pi/klipper".to_string(),
            python_path: "/home/pi/klippy-env/bin/python".to_string(),
            log_file: "/tmp/klippy.log".to_string(),
            config_file: "/home/pi/printer.cfg".to_string(),
            // process_id: 0,
            // user_id: 0,
            // group_id: 0,
        };

        let expected = MoonResponse::MoonResult {
            jsonrpc: JsonRpcVersion::V2,
            result: MoonResultData::PrinterInfoResponse(printer_info),
            id: 345
        };

        assert_eq!(response, expected);
    }
    #[tokio::test]
    async fn printer_info_parsing() {
        // let url = "ws://scanhead.local:7125/websocket";
        let hostname = std::env::var("MOONRAKER_HOSTNAME").unwrap_or("localhost".to_string());
        let port = std::env::var("MOONRAKER_PORT").unwrap_or("7125".to_string()).parse::<u16>().unwrap();
        // let mut connection = MoonConnection::new(url.to_string(), 1000, 1000).await;
        let mut connection = FastMoonConn::new(hostname, port, None, None, false).await;
        // let status = connection.get_printer_info(Some(1234)).await.unwrap();
        match connection.get_printer_info().await {
            Ok(printer_info) => {
                let state = printer_info.state;
                println!("Printer State: {:?}", state);
            },
            Err(e) => {
                eprintln!("Error getting printer info: {}", e.to_string());
                assert!(false);
            }
        }
    }
}