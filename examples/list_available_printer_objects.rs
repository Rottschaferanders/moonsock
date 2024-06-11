use moonsock::{
    // FastMoonConn,
    MoonrakerClient,
    // MoonMSG, 
    MoonRequest, MoonResponse,
    MoonMethod, response::MoonResultData
};
use std::env;

const DEFAULT_MOONRAKER_PORT: u16 = 7125;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname = env::var("MOONRAKER_HOSTNAME").expect("Please add the `MOONRAKER_HOSTNAME` environment variable");
    let port = match env::var("MOONRAKER_PORT") {
        Ok(port_string) => {
            match port_string.parse::<u16>() {
                Ok(port) => port,
                Err(_e) => DEFAULT_MOONRAKER_PORT,
            }
        },
        Err(_e) => DEFAULT_MOONRAKER_PORT,
    };

    // let url = format!("ws://{hostname}:{port}/websocket");
    // let mut connection = MoonConnection::new(url, 1000, 1000).await;
    // let mut connection = FastMoonConn::new(hostname, port, None, None, false).await?;
    // let mut connection = MoonrakerClient::new_simple(hostname, Some(port), false).await?;
    let mut connection = MoonrakerClient::new(hostname, Some(port)).await?;
    // let msg = MoonMSG::new(MoonMethod::PrinterObjectsList, None, Some(19876)); // Choose a message ID
    let msg = MoonRequest::new(MoonMethod::PrinterObjectsList, None);
    let response = connection.send_listen(msg).await?;

    match response {
        // MoonMSG::MoonResult { result, .. } => match result {
        MoonResponse::MoonResult { result, .. } => match result {
            MoonResultData::PrinterObjectsListResponse(data) => {
                println!("Available Printer Objects: {:?}", data.objects);
            }
            _ => println!("Unexpected response format"),
        },
        _ => println!("Unexpected response type"),
    }

    Ok(())
}