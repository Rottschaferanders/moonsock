use moonsock::FastMoonConn;

use std::env;

const DEFAULT_MOONRAKER_PORT: u16 = 7125;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hostname = env::var("MOONRAKER_HOSTNAME").expect("Please add the `MOONRAKER_HOSTNAME` environment variable");
    let port = match env::var("MOONRAKER_PORT") {
        Ok(port_string) => {
            match port_string.parse::<u16>() {
                Ok(port) => port,
                Err(_e) => {
                    // println!("Could not parse the `MOONRAKER_PORT` environment variable into a `u16`. Using the default port: {DEFAULT_MOONRAKER_PORT}");
                    DEFAULT_MOONRAKER_PORT
                }
            }
        },
        Err(_e) => {
            // println!("Could not find `MOONRAKER_PORT` environment variable. Using the default port: {DEFAULT_MOONRAKER_PORT}");
            DEFAULT_MOONRAKER_PORT
        },
    };

    // let url = format!("ws://{hostname}:{port}/websocket");
    // let mut connection = FastMoonConn::new(url.to_string(), 1000, 1000, false).await;
    let mut connection = FastMoonConn::new(hostname, port, None, None, false).await;

    let is_homed = connection.is_homed().await?;
    if is_homed {
        println!("Printer is homed!");
    } else {
        println!("Printer is not homed");
    }
    // match connection.get_homed_axes().await {
    //     Ok(homed_printer_axes) => {
    //         println!("homed_printer_axes: {:?}", homed_printer_axes);
    //     },
    //     Err(e) => {
    //         eprintln!("Error getting printer info: {}", e.to_string());
    //     }
    // }
    Ok(())
}