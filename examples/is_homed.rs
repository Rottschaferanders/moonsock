use moonsock::connection::PrinterSafetyStatus;
use moonsock::FastMoonConn;
use moonsock::MoonConnection;

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
    // let mut connection = FastMoonConn::new(hostname, port, None, None, true).await?;
    let mut connection = MoonConnection::new_simple(hostname, Some(port), false).await;
    println!("Connected to moonraker");
    match connection.ensure_ready().await {
        PrinterSafetyStatus::Ready => println!("Printer is ready!"),
        PrinterSafetyStatus::KlipperError(e) => eprintln!("Error: {e}"),
        PrinterSafetyStatus::OtherError(e) => eprintln!("Error: {e}"),
        PrinterSafetyStatus::Maybe3DPrintInsidePrinter(state) => eprintln!("Error: There could be a print inside the printer! Printer State: {state:?}"),
        PrinterSafetyStatus::Shutdown => eprintln!("Error: The printer is shutting down!"),
        PrinterSafetyStatus::TimeoutReached => eprintln!("Error: The printer timed out!"),
        PrinterSafetyStatus::TooManyRestarts => eprintln!("Error: The printer restarted too many times!"),
    }
    // match connection.ensure_ready().await {
    //     Ok(PrinterSafetyStatus::Ready) => println!("Printer is ready!"),
    //     Err(PrinterSafetyStatus::KlipperError(e)) => eprintln!("Error: {e}"),
    //     Err(PrinterSafetyStatus::OtherError(e)) => eprintln!("Error: {e}"),
    //     Err(PrinterSafetyStatus::Maybe3DPrintInsidePrinter(state)) => eprintln!("Error: There could be a print inside the printer! Printer State: {state:?}"),
    //     Err(PrinterSafetyStatus::Shutdown) => eprintln!("Error: The printer is shutting down!"),
    //     Err(PrinterSafetyStatus::TimeoutReached) => eprintln!("Error: The printer timed out!"),
    //     Err(PrinterSafetyStatus::TooManyRestarts) => eprintln!("Error: The printer restarted too many times!"),
    //     _ => println!("Bad variant of PrinterSafetyStatus"),
    // }
    let is_homed = match connection.is_homed().await {
        Ok(homed) => homed,
        Err(e) => {
            println!("Error getting printer info: {}", e.to_string());
            Err(e)?
        }
    };

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