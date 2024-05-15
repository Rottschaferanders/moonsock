// use moonsock::{MoonConnection, MoonMSG, MoonMethod, moon_result::MoonResultData};
// use std::env;
// use serde_json::Value;

// const DEFAULT_MOONRAKER_PORT: u16 = 7125;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let hostname = env::var("MOONRAKER_HOSTNAME").expect("Please add the `MOONRAKER_HOSTNAME` environment variable");
//     let port = match env::var("MOONRAKER_PORT") {
//         Ok(port_string) => {
//             match port_string.parse::<u16>() {
//                 Ok(port) => port,
//                 Err(_e) => DEFAULT_MOONRAKER_PORT,
//             }
//         },
//         Err(_e) => DEFAULT_MOONRAKER_PORT,
//     };

//     let url = format!("ws://{hostname}:{port}/websocket");
//     let mut connection = MoonConnection::new(url, 1000, 1000).await;

//     // Craft the query message
//     let msg = MoonMSG::new(MoonMethod::PrinterObjectsQuery, Some(serde_json::json!({
//         "objects": ["z_tilt"] 
//     })), Some(1234)); // Choose your own message ID

//     // Send the query and process response
//     match connection.send_listen(msg).await? {
//         MoonMSG::MoonResult { result, .. } => match result {
//             MoonResultData::QueryPrinterObjectsResponse(data) => {
//                 if let Some(z_tilt_status) = data.status.z_tilt {
//                     if let Some(applied) = z_tilt_status.get("applied") {
//                         if applied.as_bool().unwrap_or(false) { // Or 'applied == &Value::Bool(true)' 
//                             println!("Z-Tilt is applied");
//                         } else {
//                             println!("Z-Tilt is not applied");
//                         }
//                     } else {
//                         println!("Error: 'applied' field not found in Z-Tilt status");
//                     }
//                 } else {
//                     println!("Error: 'z_tilt' object not found in printer status");
//                 }
//             }
//             _ => println!("Unexpected response format"),
//         },
//         _ => println!("Unexpected response type"),
//     }

//     Ok(())
// }

// use moonsock::{moon_param::{MoonParam, PrinterObject}, moon_result::MoonResultData, MoonConnection, MoonMSG, MoonMethod};
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
                Err(_e) => DEFAULT_MOONRAKER_PORT,
            }
        },
        Err(_e) => DEFAULT_MOONRAKER_PORT,
    };

    // let url = format!("ws://{hostname}:{port}/websocket");
    // let mut connection = MoonConnection::new(url, 1000, 1000).await;
    let mut connection = FastMoonConn::new(hostname, port, None, None, false).await;
    
    let is_z_tilt_appled = connection.is_z_tilt_applied().await?;
    if is_z_tilt_appled {
        println!("Z-Tilt is applied!");
    } else {
        println!("Z-Tilt is not applied");
    }

    // let msg = MoonMSG::new(
    //     MoonMethod::PrinterObjectsQuery, 
    //     Some(MoonParam::PrinterObjectsQuery {
    //         objects: PrinterObject::ZTilt(None),
    //     }), 
    //     Some(19876), // Choose a message ID
    // ); 

    // match connection.send_listen(msg).await? {
    //     MoonMSG::MoonResult { result, .. } => match result {
    //         MoonResultData::QueryPrinterObjectsResponse(data) => {
    //             if let Some(z_tilt) = data.status.z_tilt {
    //                 if z_tilt.applied {
    //                     println!("Z-Tilt is applied!");
    //                 } else {
    //                     println!("Z-Tilt is not applied");
    //                 }
    //             } else {
    //                 println!("Z-Tilt object not found in printer response."); 
    //             }
    //         }
    //         _ => println!("Unexpected response format"),
    //     },
    //     _ => println!("Unexpected response type"),
    // }

    Ok(())
}