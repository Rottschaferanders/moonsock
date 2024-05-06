use moonsock::MoonConnection;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://scanhead.local:7125/websocket";
    let mut connection = MoonConnection::new(url.to_string(), 1000, 1000).await;
    // let status = connection.get_printer_info(Some(1234)).await.unwrap();
    match connection.get_homed_axes().await {
        Ok(homed_printer_axes) => {
            // let state = printer_info;
            println!("homed_printer_axes: {:?}", homed_printer_axes);
        },
        Err(e) => {
            eprintln!("Error getting printer info: {}", e.to_string());
            assert!(false);
        }
    }
    Ok(())
}