use moonsock::MoonConnection;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://scanhead.local:7125/websocket";
    let mut connection = MoonConnection::new(url.to_string(), 1000, 1000).await;
    // let status = connection.get_printer_info(Some(1234)).await.unwrap();
    match connection.check_printer_homed().await {
        Ok(is_printer_homed) => {
            // let state = printer_info;
            println!("Printer Home Status: {:?}", is_printer_homed);
        },
        Err(e) => {
            eprintln!("Error getting printer info: {}", e.to_string());
            assert!(false);
        }
    }
    Ok(())
}