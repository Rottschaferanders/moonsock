// use moonsock::{ MoonConnection, messages::PrinterInfoResponse, messages::PrinterState};
use moonsock::MoonConnection;

#[tokio::test]
async fn printer_info_parsing() {
    let url = "ws://scanhead.local:7125/websocket";
    let mut connection = MoonConnection::new(url.to_string(), 1000, 1000).await;
    // let status = connection.get_printer_info(Some(1234)).await.unwrap();
    match connection.get_printer_info(Some(1234)).await {
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