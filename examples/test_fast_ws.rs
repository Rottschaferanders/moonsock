// use moonsock::MoonConnection;
use std::env;

use moonsock::{
    // fast_ws_stuff::connect,
    fast_ws_connection::FastMoonConn, MoonMethod, MoonRequest
};

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
    // let mut connection = MoonConnection::new(url, None, None, false).await;
    let mut conn = FastMoonConn::new_new(hostname, port, None, None, false).await;
    for i in 0..10 {
        let req = MoonRequest::new(MoonMethod::PrinterInfo, None);
        let res = conn.send_listen(req).await?;
        println!("Res: {res:?}");
    }

    // let conn = connect(hostname, port).await?;
    

    Ok(())
}