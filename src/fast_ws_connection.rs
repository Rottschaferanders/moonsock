// use tokio::{
//     io::{
//         self, AsyncWriteExt, 
//         // AsyncReadExt,
//     },
//     // io::AsyncReadExt, 
//     net::TcpStream, 
//     sync::mpsc::{
//         error::SendError,
//         Permit,
//     }, 
//     time::timeout
// };
// use tokio_util::codec::{FramedRead, LinesCodec};
// use futures::StreamExt;

// use fastwebsockets::{
//     // FragmentCollector, 
//     Payload, Frame,
//     handshake, WebSocket, WebSocketError, OpCode,
// };
// use hyper::{Request, body::Bytes, upgrade::Upgraded, header::{UPGRADE, CONNECTION}};
// use hyper_util::rt::TokioIo;
// use hyper_util::rt::tokio::TokioExecutor;
// use http_body_util::Empty;
// // use std::future::Future;
// // use anyhow::Result;
// use url::Url;

// use crate::{
//     // fast_ws_stuff::connect, 
//     response::{ServerInfo, PrinterInfoResponse, PrinterState}, 
//     MoonResultData,
//     // MoonErrorContent, 
//     MoonMethod, MoonParam, MoonRequest, MoonResponse, 
//     // NotificationMethod, 
//     // PrinterInfoResponse, 
//     PrinterObject, 
//     // response::PrinterState,
//     connection::MoonSendError,
// };

// const DEFAULT_WRITER_BUFFER_SIZE: usize = 1000;
// const DEFAULT_READER_BUFFER_SIZE: usize = 1000;

// const TCP_CONNECT_MOONRAKER_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(15);

// async fn try_connect(hostname: String, port: u16) -> Result<WebSocket<TokioIo<Upgraded>>, Box<dyn std::error::Error>> {
//     let url = format!("ws://{hostname}:{port}/websocket");
//     let connect_addr = Url::parse(&url).unwrap();
//     let domain = connect_addr.domain().unwrap();
//     let port = connect_addr
//         .port()
//         .or_else(|| match connect_addr.scheme() {
//             "wss" => Some(443),
//             "ws" => Some(80),
//             _ => None,
//         }).expect("Failed to figure out what port you wanted");

//     let addr = format!("{domain}:{port}");

//     let connect_future = async move {
//         loop {
//             match TcpStream::connect(addr.clone()).await {
//                 Ok(stream) => {
//                     // Perform the handshake
//                     let req = Request::builder()
//                         .method("GET")
//                         .uri("/websocket")
//                         .header("Host", hostname)
//                         .header(UPGRADE, "websocket")
//                         .header(CONNECTION, "upgrade")
//                         .header(
//                             "Sec-WebSocket-Key",
//                             fastwebsockets::handshake::generate_key(),
//                         )
//                         .header("Sec-WebSocket-Version", "13")
//                         .body(Empty::<Bytes>::new()).unwrap();

//                     let (ws, _) = handshake::client(&TokioExecutor::new(), req, stream).await.unwrap();
//                     println!("Websocket Succesfully connected!");
//                     return Ok(ws);
//                 }
//                 Err(_) => {
//                     // Sleep for some time before trying again
//                     tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//                 }
//             }
//         }
//     };

//     timeout(TCP_CONNECT_MOONRAKER_TIMEOUT, connect_future).await?
// }

// async fn ask_user_if_they_want_to_restart_printer() -> Result<bool, Box<dyn std::error::Error>> {
//     // println!("The printer is in an error or shutdown state. Do you want to restart the printer? (y/n)");
//     // let mut input = String::new();
//     // // std::io::stdin().read_line(&mut input).expect("Failed to read input");
//     // // let _ = tokio::io::stdin().read_to_string(&mut input).expect("Failed to read input");
//     // let _ = tokio::io::stdin().read_to_string(&mut input);
//     // input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes"

//     let mut stdout = io::stdout();
//     loop {
//         stdout.write_all(b"The printer is in an error or shutdown state. Do you want to restart the printer? (y/n)").await?;
//         stdout.flush().await?;

//         let stdin = io::stdin();
//         let mut reader = FramedRead::new(stdin, LinesCodec::new());
//         let input = reader.next().await.transpose()?.unwrap();
//         println!("input: {}", input);
//         if input.trim().to_lowercase() == "y" {
//             // break; // Nozzle is clean, continue
//             return Ok(true);
//         } else {
//             stdout.write_all(b"You need to make sure it's cleaned before continuing to avoid damaging your printer.\n").await?;
//             stdout.flush().await?;
//         }
//     }
// }

// /// A WebSocket connection to a Moonraker server.
// pub struct FastMoonConn {
//     write_stream: tokio::sync::mpsc::Sender<MoonRequest>,
//     read_stream: tokio::sync::mpsc::Receiver<MoonResponse>,
//     shutdown_sender: tokio::sync::mpsc::Sender<bool>,
// }
// impl FastMoonConn {
//     pub async fn new(host: String, port: u16, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Result<Self, Box<dyn std::error::Error>> {
//         let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
//         let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);
        
//         let (ws_writer_sender,      mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
//         let (shutdown_sender,              mut shutdown_receiver) = tokio::sync::mpsc::channel(10);
    
//         let mut ws_stream = try_connect(host.clone(), port).await.unwrap();
        
//         if debug {
//             println!("WebSocket handshake has been successfully completed");
//         }
    
//         let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
    
//         tokio::spawn(async move {
//             loop {
//                 if ws_stream.is_closed() {
//                     println!("Websocket connection detected closed by the `.is_closed()` method");
//                     // Reconnect to the websocket
//                     ws_stream = try_connect(host.clone(), port).await.unwrap();
//                 }
//                 // if ws_stream.is_closed() {
//                 //     println!("WebSocket connection is closed");
//                 //     // Reconnect to the websocket
//                 //     ws_stream = try_connect(host.clone(), port).await.unwrap();
//                 //     // break;
//                 // } else {
//                 // println!("Websocket connection is still open");
//                 match shutdown_receiver.try_recv() {
//                     Ok(should_shutdown) => {
//                         if should_shutdown {
//                             break;
//                         }
//                     },
//                     Err(_) => {},
//                 }
//                 match ws_writer_receiver.recv().await {
//                     Some(msg) => {
//                         println!("Trying to write a message to websocket");
//                         let mut vec = serde_json::to_vec(&msg).expect("Could not Serialize Request");
//                         vec.truncate(vec.len());
//                         let frame = Frame::binary(Payload::Owned(vec));
//                         match ws_stream.write_frame(frame).await {
//                             Ok(_) => {
//                                 println!("Wrote frame to websocket");
//                             },
//                             Err(_) => println!("Unable to send to moon_socket_sink"),
//                         }
//                     },
//                     None => {},
//                 }
//                 match ws_stream.read_frame().await {
//                     Ok(frame) => {
//                         if debug {
//                             println!("{:#?}", frame.payload);
//                         }
//                         match frame.opcode {
//                             OpCode::Continuation => {
//                                 println!("Continuation frame received");
//                             },
//                             OpCode::Text | OpCode::Binary => {
//                                 println!("Got frame from websocket");
//                                 let payload = frame.payload;
//                                 let message = match payload {
//                                     Payload::BorrowedMut(contents) => {
//                                         let res_str = String::from_utf8(contents.to_vec()).unwrap();
//                                         res_str
//                                     },
//                                     Payload::Borrowed(contents) => {
//                                         let res_str = String::from_utf8(contents.to_vec()).unwrap();
//                                         res_str
//                                     },
//                                     Payload::Bytes(contents) => {
//                                         let res_str = String::from_utf8(contents.to_vec()).unwrap();
//                                         res_str
//                                     },
//                                     Payload::Owned(contents) => {
//                                         let res_str = String::from_utf8(contents.to_vec()).unwrap();
//                                         res_str
//                                     }
//                                 };

//                                 if debug {
//                                     println!("{:?}", message);
//                                 }
            
//                                 match serde_json::from_str::<MoonResponse>(message.as_str()) {
//                                     Ok(response) => {
//                                         match ws_reader_sender.send(response).await {
//                                             Ok(()) => continue,
//                                             Err(e) => println!("Unable to send to ws_reader_sender: {}", e.to_string()),
//                                         }
//                                     },
//                                     Err(_) => {
//                                         println!("----------------------------MESSAGE NOT PARSED----------------------------");
//                                         println!("Message Length: {}", message.len());
//                                         println!("{}", message);
//                                         println!("--------------------------------------------------------------------------");
//                                     }
//                                 }
//                             },
//                             // OpCode::Binary => {
//                             // },
//                             OpCode::Close => {
//                                 println!("Websocket Close frame received");
//                             },
//                             OpCode::Ping => {
//                                 println!("Ping");
//                             },
//                             OpCode::Pong => {
//                                 println!("Pong");
//                             },
//                         }
//                         // println!("Got frame from websocket");
//                         // let payload = frame.payload;
//                         // let message = match payload {
//                         //     Payload::BorrowedMut(contents) => {
//                         //         let res_str = String::from_utf8(contents.to_vec()).unwrap();
//                         //         res_str
//                         //     },
//                         //     Payload::Borrowed(contents) => {
//                         //         let res_str = String::from_utf8(contents.to_vec()).unwrap();
//                         //         res_str
//                         //     },
//                         //     Payload::Bytes(contents) => {
//                         //         let res_str = String::from_utf8(contents.to_vec()).unwrap();
//                         //         res_str
//                         //     },
//                         //     Payload::Owned(contents) => {
//                         //         let res_str = String::from_utf8(contents.to_vec()).unwrap();
//                         //         res_str
//                         //     }
//                         // };
    
//                         // match serde_json::from_str::<MoonResponse>(message.as_str()) {
//                         //     Ok(response) => {
//                         //         match ws_reader_sender.send(response).await {
//                         //             Ok(()) => continue,
//                         //             Err(e) => println!("Unable to send to ws_reader_sender: {}", e.to_string()),
//                         //         }
//                         //     },
//                         //     Err(_) => {
//                         //         println!("----------------------------MESSAGE NOT PARSED----------------------------");
//                         //         println!("Message Length: {}", message.len());
//                         //         println!("{}", message);
//                         //         println!("--------------------------------------------------------------------------");
//                         //     }
//                         // }
//                     },
//                     Err(e) => {
//                         eprintln!("Error while reading frame: {}", e.to_string());
//                         // Reconnect to the websocket
//                         // ws_stream = try_connect(host.clone(), port).await.unwrap();
//                         if let WebSocketError::ConnectionClosed = e {
//                             // Reconnect to the websocket
//                             ws_stream = try_connect(host.clone(), port).await.unwrap();
//                         } else {
//                             eprintln!("Error while reading frame: {}", e.to_string());
//                         }
//                     }
//                 }
//                 // }
//             }
//         });
    
//         // let mut connection = Self {
//         //     write_stream: ws_writer_sender,
//         //     read_stream: ws_reader_receiver,
//         //     shutdown_sender,
//         // };
    
//         // ... rest of the function

//         // loop {
//         //     let server_info = connection.get_server_info().await?;
//         //     println!("Got server_info: {server_info:?}");
//         //     match server_info.klippy_state {
//         //         PrinterState::Ready => {
//         //             println!("Printer is ready");
//         //             break;
//         //         }
//         //         PrinterState::Startup => {
//         //             // Wait for 5 seconds and then check the printer state again
//         //             tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//         //         }
//         //         PrinterState::Shutdown | PrinterState::Error => {
//         //             if ask_user_if_they_want_to_restart_printer() {
//         //                 let message = MoonRequest::new(MoonMethod::PrinterFirmwareRestart, None);
//         //                 // connection.send_wait_for_ok(&message).await?;
//         //                 connection.send(&message).await?;
//         //                 println!("Sent restart command, waiting 10s and then checking server_info again");
//         //                 // Wait for 10 seconds to allow the printer to restart
//         //                 tokio::time::sleep(std::time::Duration::from_secs(10)).await;
//         //             } else {
//         //                 // Handle the case where the user does not want to restart the printer
//         //             }
//         //         }
//         //         _ => {
//         //             // Wait for some time and try again
//         //             tokio::time::sleep(std::time::Duration::from_secs(2)).await;
//         //         }
//         //     }
//         // }

//         // println!("Printer is ready");
//         // Ok(connection)
//         Ok(Self {
//             write_stream: ws_writer_sender,
//             read_stream: ws_reader_receiver,
//             shutdown_sender,
//         })
//     }

//     pub async fn ensure_ready(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
//         let timeout = tokio::time::Duration::from_secs(40);
//         let mut start_time = tokio::time::Instant::now();
//         let mut ask_user_when = tokio::time::Instant::now();
        
//         loop {
//             if start_time.elapsed() > timeout {
//                 return Err("Printer did not ready before timout was reached".into());
//             }
//             let server_info = self.get_server_info().await?;
//             println!("Got server_info: {server_info:?}");
//             match server_info.klippy_state {
//                 PrinterState::Ready => {
//                     println!("Printer is ready");
//                     // break;
//                     return Ok(true);
//                 },
//                 PrinterState::Startup => {
//                     println!("Printer is starting up");
//                     // Wait for 5 seconds and then check the printer state again
//                     // tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//                 },
//                 PrinterState::Shutdown | PrinterState::Error => {
//                     println!("Printer is in error or shutdown state");
//                     if tokio::time::Instant::now() > ask_user_when {
//                         if ask_user_if_they_want_to_restart_printer().await? {
//                             let message = MoonRequest::new(MoonMethod::PrinterFirmwareRestart, None);
//                             // connection.send_wait_for_ok(&message).await?;
//                             self.send(&message).await?;
//                             start_time = tokio::time::Instant::now();
//                             println!("Sent restart command, waiting 20s and then checking server_info again");
//                             // Wait for 10 seconds to allow the printer to restart
//                             tokio::time::sleep(std::time::Duration::from_secs(20)).await;
//                         } else {
//                             // Handle the case where the user does not want to restart the printer
//                         }
//                         ask_user_when = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
//                     }
//                     // if ask_user_if_they_want_to_restart_printer().await? {
//                     //     let message = MoonRequest::new(MoonMethod::PrinterFirmwareRestart, None);
//                     //     // connection.send_wait_for_ok(&message).await?;
//                     //     self.send(&message).await?;
//                     //     start_time = tokio::time::Instant::now();
//                     //     println!("Sent restart command, waiting 20s and then checking server_info again");
//                     //     // Wait for 10 seconds to allow the printer to restart
//                     //     tokio::time::sleep(std::time::Duration::from_secs(20)).await;
//                     // } else {
//                     //     // Handle the case where the user does not want to restart the printer
//                     // }
//                 },
//                 _ => {
//                     println!("Printer is in a state that is not Ready, Startup, Shutdown, or Error");
//                     // Wait for some time and try again
//                     // tokio::time::sleep(std::time::Duration::from_secs(2)).await;
//                 },
//             }
//         }
//     }

//     pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         match self.shutdown_sender.send(true).await {
//             Ok(_) => {
//                 Ok(())
//             },
//             Err(e) => {
//                 Err(format!("Failed to send shutdown signal to the moonraker message stream: {}", e.to_string()).into())
//             },
//         }
//     }
//     /// The most basic of the sender methods. Adds a message to the WebSocket writer queue to be sent to the Moonraker instance.
//     /// This function is non-blocking and returns immediately after the message is sent.
//     /// When the websocket writer queue is full, this function will not block, but will yield back to the parent function until a spot opens up in the queue which
//     /// can be problematic if the order of messages you're sending to the printer is important. In that case, consider using the `send_reserved` function instead. 
//     ///
//     /// # Arguments
//     ///
//     /// * `message` - The `MoonMSG` message to send.
//     ///
//     /// # Returns
//     ///
//     /// Returns `Ok(id)` which is the new randomly selected message id, or a `SendError<MoonMSG>`
//     pub async fn send(&mut self, message: &MoonRequest) -> Result<u32, Box<dyn std::error::Error>> {
//         // self.write_stream.send(message).await.map_err(|e| e.into())?;
//         let this_id = rand::random();
//         let mut msg = message.clone();
//         msg.id = this_id;
//         match self.write_stream.send(msg).await {
//             Ok(()) => Ok(this_id),
//             Err(e) => Err(format!("Error sending `{:?}` to the `FastMoonConn` request channel: {}", e.0, e.to_string()).into())
//         }
//         // Ok(())
//     }
    
//     /// Sends a message over the WebSocket connection, using a reserved spot in the writer queue.
//     ///
//     /// This function is similar to `send`, but it uses a reserved spot in the writer queue, so it
//     /// doesn't block the thread if the queue is full, but also ensures that the order of messages is exactly as your program describes. 
//     /// Use this function if you are sending a lot of messages in a short amount of time and the order of those messages matters.
//     /// 
//     /// Essentially, you are putting a dynamic buffer on top of the fixed-sized primary message buffer to ensure that messages are sent in the order you want.
//     /// 
//     /// For example, say you wanted to ensure the printer recieved a `G28` (Home Printer) command before a `G1 Z10` (move printer bed up) command. 
//     /// Although, in this case, you'll want to sleep for a bit after the `G28` command to ensure the printer has time to home before moving the bed up.
//     ///
//     /// # Arguments
//     ///
//     /// * `message` - The message to send over the WebSocket connection.
//     ///
//     /// # Returns
//     ///
//     /// A `Result` indicating whether the message was successfully sent or not. An error here indicates that the websocket channel is probably closed.
//     pub async fn send_reserved(&mut self, message: MoonRequest) -> Result<(), MoonSendError<()>> {
//         // let permit = self.reserve().await.map_err(|e| e.into())?;
//         let permit = self.reserve().await.map_err(|e| MoonSendError::SendError(e))?;
//         permit.send(message);
//         Ok(())
//     }
//     /// Reserves a permit from the WebSocket writer queue for sending a message.
//     ///
//     /// # Returns
//     ///
//     /// Returns a `Permit<MoonMSG>` if a permit was successfully reserved, or a `SendError<()>` if the connection has closed.
//     // pub async fn reserve(&self) -> Result<Permit<MoonMSG>, SendError<()>> {
//     pub async fn reserve(&self) -> Result<Permit<MoonRequest>, SendError<()>> {
//         self.write_stream.reserve().await
//     }
//     /// Waits for a message to be received from the Moonraker instance.
//     ///
//     /// # Returns
//     ///
//     /// Returns an `Option<MoonMSG>` containing the received message, or `None` if the receiver channel has been closed.
//     pub async fn recv(&mut self) -> Option<MoonResponse> {
//         self.read_stream.recv().await
//     }
//     /// Sends message and then waits for the printer to send an Ok message back
//     // pub async fn send_wait_for_ok(&mut self, message: &MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
//     //     // let this_id = message.id;
//     //     let this_id = self.send(&message).await?;
//     //     loop {
//     //         match self.recv().await {
//     //             Some(msg) => {
//     //                 match msg {
//     //                     // MoonMSG::MoonResult { id, result, .. } => {
//     //                     MoonResponse::MoonResult { id, result, .. } => {
//     //                         if id == this_id {
//     //                             match result {
//     //                                 MoonResultData::Ok(..) => {
//     //                                     return Ok(());
//     //                                 },
//     //                                 _ => continue,
//     //                             }
//     //                         } else {
//     //                             println!("Ids did not match");
//     //                         }
//     //                     },
//     //                     _ => continue,
//     //                 }
//     //             },
//     //             None => continue,
//     //         }
//     //     }
//     // }
//     // pub async fn send_wait_for_ok(&mut self, message: &MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
//     //     let this_id = self.send(&message).await?;
//     //     println!("Sending message with id: {this_id}");
//     //     let timeout = tokio::time::Duration::from_secs(10);
//     //     let start_time = tokio::time::Instant::now();
//     //     loop {
//     //         match self.recv().await {
//     //             Some(res) => {
//     //                 println!("Got a response: {}", serde_json::to_string_pretty(&res).unwrap());
//     //                 match res {
//     //                     MoonResponse::MoonResult { result, id, .. } => {
//     //                         if id == this_id {
//     //                             match result {
//     //                                 MoonResultData::Ok(_) => return Ok(()),
//     //                                 _ => return Err("Did not receive ok response".into()),
//     //                             }
//     //                         }
//     //                     },
//     //                     MoonResponse::MoonError { error, id, .. } => {
//     //                         match id {
//     //                             Some(id) => {
//     //                                 if id == this_id {
//     //                                     return Err(error.into());
//     //                                     // return Err(Box::new(error));
//     //                                 }
//     //                             },
//     //                             None => continue,
//     //                         }
//     //                     },
//     //                     MoonResponse::Notification { .. } => {},
//     //                 }
//     //             },
//     //             None => continue,
//     //         }
//     //         if start_time.elapsed() > timeout {
//     //             return Err("Timeout waiting for response".into());
//     //         }
//     //     }
//     // }
//     // pub async fn send_wait_for_ok(&mut self, message: &MoonRequest) -> Result<(), MoonSendError<MoonRequest>> {
//     //     let this_id = self.send(&message).await?;
//     //     println!("Sending message with id: {this_id}");
//     //     let timeout = tokio::time::Duration::from_secs(10);
//     //     let start_time = tokio::time::Instant::now();
//     //     loop {
//     //         match self.recv().await {
//     //             Some(res) => {
//     //                 println!("Got a response: {}", serde_json::to_string_pretty(&res).unwrap());
//     //                 match res {
//     //                     MoonResponse::MoonResult { result, id, .. } => {
//     //                         if id == this_id {
//     //                             match result {
//     //                                 MoonResultData::Ok(_) => return Ok(()),
//     //                                 _ => return Err(MoonSendError::MoonResult(result)),
//     //                             }
//     //                         }
//     //                     },
//     //                     MoonResponse::MoonError { error, id, .. } => {
//     //                         match id {
//     //                             Some(id) => {
//     //                                 if id == this_id {
//     //                                     return Err(MoonSendError::MoonError(error));
//     //                                 }
//     //                             },
//     //                             None => continue,
//     //                         }
//     //                     },
//     //                     MoonResponse::Notification { .. } => {},
//     //                 }
//     //             },
//     //             None => continue,
//     //         }
//     //         if start_time.elapsed() > timeout {
//     //             return Err(MoonSendError::SendError(tokio::sync::mpsc::error::SendError(message.clone())));
//     //         }
//     //     }
//     // }
//     pub async fn send_wait_for_ok(&mut self, message: &MoonRequest) -> Result<(), MoonSendError<MoonRequest>> {
//         let this_id = self.send(&message).await?;
//         println!("Sending message with id: {this_id}");
//         let timeout = tokio::time::Duration::from_secs(10);
//         let start_time = tokio::time::Instant::now();
//         loop {
//             match self.recv().await {
//                 Some(res) => {
//                     println!("Got a response: {}", serde_json::to_string_pretty(&res).unwrap());
//                     match res {
//                         MoonResponse::MoonResult { result, id, .. } => {
//                             if id == this_id {
//                                 match result {
//                                     MoonResultData::Ok(_) => return Ok(()),
//                                     _ => return Err(MoonSendError::String("Did not receive an Ok Response but expected one.".to_string())),
//                                 }
//                             }
//                         },
//                         MoonResponse::MoonError { error, id, .. } => {
//                             match id {
//                                 Some(id) => {
//                                     if id == this_id {
//                                         return Err(MoonSendError::MoonError(error));
//                                     }
//                                 },
//                                 None => continue,
//                             }
//                         },
//                         MoonResponse::Notification { .. } => continue, // Ignore notifications
//                     }
//                 },
//                 None => continue,
//             }
//             if start_time.elapsed() > timeout {
//                 return Err(MoonSendError::SendError(tokio::sync::mpsc::error::SendError(message.clone())));
//             }
//         }
//     }
    
//     // pub async fn send_listen(&mut self, message: &MoonRequest) -> Result<MoonResponse, Box<dyn std::error::Error>> {
//     //     // let this_id = rand::random();
//     //     // let mut msg = message.clone();
//     //     // msg.id = this_id;
//     //     // let this_id = message.id;
//     //     let this_id = self.send(&message).await?;
//     //     println!("Sending message with id: {this_id}");
//     //     loop {
//     //         match self.recv().await {
//     //             Some(res) => {
//     //                 println!("Got a response: {}", serde_json::to_string_pretty(&res).unwrap());
//     //                 match res {
//     //                     MoonResponse::MoonResult { result, id, .. } => {
//     //                         if id == this_id {
//     //                             return Ok(MoonResponse::MoonResult { jsonrpc: crate::JsonRpcVersion::V2, result, id });
//     //                         }
//     //                     },
//     //                     MoonResponse::MoonError { error, id, .. } => {
//     //                         match id {
//     //                             Some(id) => {
//     //                                 if id == this_id {
//     //                                     return Ok(MoonResponse::MoonError { jsonrpc: crate::JsonRpcVersion::V2, error, id: Some(id) });
//     //                                 }
//     //                             },
//     //                             None => continue,
//     //                         }
//     //                     },
//     //                     MoonResponse::Notification { .. } => {},
//     //                 }
//     //             },
//     //             None => continue,
//     //         }
//     //     }
//     // }
//     // pub async fn send_listen(&mut self, message: &MoonRequest) -> Result<MoonResponse, Box<dyn std::error::Error>> {
//     //     let this_id = self.send(&message).await?;
//     //     println!("Sending message with id: {this_id}");
//     //     let timeout = tokio::time::Duration::from_secs(10);
//     //     let start = tokio::time::Instant::now();
//     //     loop {
//     //         tokio::select! {
//     //             res = self.recv() => {
//     //                 println!("Got a response: {}", serde_json::to_string_pretty(&res).unwrap());
//     //                 match res {
//     //                     Some(res) => {
//     //                         match res {
//     //                             MoonResponse::MoonResult { result, id, .. } => {
//     //                                 if id == this_id {
//     //                                     return Ok(MoonResponse::MoonResult { jsonrpc: crate::JsonRpcVersion::V2, result, id });
//     //                                 }
//     //                             },
//     //                             MoonResponse::MoonError { error, id, .. } => {
//     //                                 match id {
//     //                                     Some(id) => {
//     //                                         if id == this_id {
//     //                                             return Ok(MoonResponse::MoonError { jsonrpc: crate::JsonRpcVersion::V2, error, id: Some(id) });
//     //                                         }
//     //                                     },
//     //                                     None => continue,
//     //                                 }
//     //                             },
//     //                             MoonResponse::Notification { .. } => {},
//     //                         }
//     //                     },
//     //                     None => continue,
//     //                 }
//     //             },
//     //             _ = tokio::time::sleep(timeout) => {
//     //                 return Err(format!("Timeout waiting for response to message with id {}", this_id).into());
//     //             }
//     //         }
//     //         if start.elapsed() > timeout {
//     //             return Err(format!("Timeout waiting for response to message with id {}", this_id).into());
//     //         }
//     //     }
//     // }

//     pub async fn send_listen(&mut self, message: &MoonRequest) -> Result<MoonResponse, Box<dyn std::error::Error>> {
//         let this_id = self.send(&message).await?;
//         println!("Sending message with id: {this_id}");
//         let timeout = tokio::time::Duration::from_secs(10);
//         let start_time = tokio::time::Instant::now();
//         loop {
//             match self.recv().await {
//                 Some(res) => {
//                     println!("Got a response: {}", serde_json::to_string_pretty(&res).unwrap());
//                     match res {
//                         MoonResponse::MoonResult { result, id, .. } => {
//                             if id == this_id {
//                                 return Ok(MoonResponse::MoonResult { jsonrpc: crate::JsonRpcVersion::V2, result, id });
//                             }
//                         },
//                         MoonResponse::MoonError { error, id, .. } => {
//                             match id {
//                                 Some(id) => {
//                                     if id == this_id {
//                                         return Ok(MoonResponse::MoonError { jsonrpc: crate::JsonRpcVersion::V2, error, id: Some(id) });
//                                     }
//                                 },
//                                 None => continue,
//                             }
//                         },
//                         MoonResponse::Notification { .. } => {},
//                     }
//                 },
//                 None => continue,
//             }
//             if start_time.elapsed() > timeout {
//                 return Err("Timeout waiting for response".into());
//             }
//         }
//     }

//     pub async fn get_server_info(&mut self) -> Result<ServerInfo, Box<dyn std::error::Error>> {
//         let message = MoonRequest::new(MoonMethod::ServerInfo, None);
//         let res = self.send_listen(&message).await?;
//         match res {
//             MoonResponse::MoonResult { result, .. } => {
//                 match result {
//                     MoonResultData::ServerInfo(server_info) => {
//                         Ok(server_info)
//                     },
//                     _ => {
//                         Err(format!("Unexpected response: {:?}", result).into())
//                     },
//                 }
//             },
//             _ => {
//                 Err(format!("Unexpected response: {:?}", res).into())
//             },
//         }
//     }
//     // pub async fn get_server_info(&mut self) -> Result<ServerInfo, Box<dyn std::error::Error>> {
//     //     let message = MoonRequest::new(MoonMethod::ServerInfo, None);
//     //     match self.send_listen(&message).await {
//     //         Ok(res) => {
//     //             match res {
//     //                 MoonResponse::MoonResult { result, .. } => {
//     //                     match result {
//     //                         MoonResultData::ServerInfo(server_info) => {
//     //                             Ok(server_info)
//     //                         },
//     //                         _ => {
//     //                             Err(format!("Unexpected response: {:?}", result).into())
//     //                         },
//     //                     }
//     //                 },
//     //                 _ => {
//     //                     Err(format!("Unexpected response: {:?}", res).into())
//     //                 },
//     //             }
//     //         },
//     //         Err(e) => {
//     //             Err(e.into())
//     //         }
//     //     }
//     // }
//     pub async fn get_printer_info(&mut self) -> Result<PrinterInfoResponse, Box<dyn std::error::Error>> {
//         let message = MoonRequest::new(MoonMethod::PrinterInfo, None);
//         let res = self.send_listen(&message).await?;
//         match res {
//             MoonResponse::MoonResult { result, .. } => {
//                 match result {
//                     MoonResultData::Ok(_) => Err("Recived an ok() response from the server, but was expecting ".into()),
//                     MoonResultData::PrinterInfoResponse(printer_info) => {
//                         return Ok(printer_info);
//                     },
//                     _ => Err("Error in `FastMoonConn::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.".into())
//                 }
//             },
//             _ => Err("Error in `FastMoonConn::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.".into()),
//         }
//     }
//     pub async fn get_homed_axes(&mut self) -> Result<String, Box<dyn std::error::Error>> {
//         let param = MoonParam::PrinterObjectsQuery{
//             objects: PrinterObject::Toolhead(Some(vec!["homed_axes".to_string()])),
//         };
//         let msg = MoonRequest::new(MoonMethod::PrinterObjectsQuery, Some(param));

//         // match self.send_listen(&msg).await? {
//         //     MoonResponse::MoonResult { result, .. } => {
//         //         match result {
//         //             MoonResultData::PrinterObjectsQueryResponse(res) => {
//         //                 match res.status.toolhead {
//         //                     Some(toolhead) => {
//         //                         match toolhead.homed_axes {
//         //                             Some(homed_axes) => Ok(homed_axes),
//         //                             None => Err("Error: Could not find `homed_axes` in response from printer".into()),
//         //                         }
//         //                     },
//         //                     None => Err("Error: Could not find the `toolhead` field in response from printer".into()),
//         //                 }
//         //             },
//         //             _ => Err("Error: Printer did not return expected response".into()),
//         //         }
//         //     },
//         //     _ => Err("Error: Printer did not return expected response".into()),
//         // }
//         match self.send_listen(&msg).await {
//             Ok(res) => {
//                 match res {
//                     MoonResponse::MoonResult { result, .. } => {
//                         match result {
//                             MoonResultData::PrinterObjectsQueryResponse(res) => {
//                                 match res.status.toolhead {
//                                     Some(toolhead) => {
//                                         match toolhead.homed_axes {
//                                             Some(homed_axes) => Ok(homed_axes),
//                                             None => Err("Error: Could not find `homed_axes` in response from printer".into()),
//                                         }
//                                     },
//                                     None => Err("Error: Could not find the `toolhead` field in response from printer".into()),
//                                 }
//                             },
//                             _ => Err("Error: Printer did not return expected response".into()),
//                         }
//                     },
//                     _ => Err("Error: Printer did not return expected response".into()),
//                 }
//             },
//             Err(e) => Err(format!("Error sending message: {}", e.to_string()).into()),
//         }
//     }
//     pub async fn is_homed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
//         let homed_axes = self.get_homed_axes().await?;
//         // Assuming 'XYZ' or similar indicates all required axes are homed 
//         Ok(homed_axes.to_lowercase().contains("xyz")) 
//     }
    
//     pub async fn is_z_tilt_applied(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
//         let param = MoonParam::PrinterObjectsQuery {
//             objects: PrinterObject::ZTilt(None), 
//         };
//         let msg = MoonRequest::new(MoonMethod::PrinterObjectsQuery, Some(param));

//         match self.send_listen(&msg).await? {
//             MoonResponse::MoonResult { result, .. } => match result {
//                 MoonResultData::PrinterObjectsQueryResponse(res) => {
//                     match res.status.z_tilt {
//                         Some(z_tilt) => Ok(z_tilt.applied),
//                         None => Err("Error: 'z_tilt' object not found in response".into()),
//                     }
//                 }
//                 _ => Err("Error: Unexpected response format from Moonraker".into()),
//             },
//             _ => Err("Error: Unexpected response type from Moonraker".into()),
//         }
//     }
// }