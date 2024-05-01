use moonsock::{MoonMSG, MoonMethod, MoonParam, MoonConnection, PrinterInfoResponse, messages::PrinterState};

#[tokio::test]
async fn printer_info_parsing() {
    let url = "ws://scanhead.local:7125/websocket";
    let mut connection = MoonConnection::new(url.to_string(), 1000, 1000).await;
    let status = connection.get_printer_info(Some(1234)).await.unwrap();
    let state = status.state;
    println!("Printer State: {:?}", state);

}