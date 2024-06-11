use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::collections::HashMap;
use std::error::Error as StdError;

use std::time::{Duration, Instant};
use tokio::{
    time::sleep,
    io::{
        AsyncWriteExt, 
        stdin, stdout,
    },
    sync::{
        mpsc::{
            self,
            error::SendError,
            Permit,
        },
        oneshot,
        Mutex,
    },
};
use tokio_util::codec::{FramedRead, LinesCodec};
use spinoff::{Spinner, spinners, Color};


use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use core::pin::Pin;
use futures_util::{sink::*, StreamExt};
use url::Url;

use crate::{
    response::MoonResultData, 
    MoonErrorContent, MoonMethod, MoonParam, MoonRequest, MoonResponse, 
    response::{PrinterState, ServerInfo, PrinterInfoResponse}, 
    PrinterObject,
    PrinterSafetyStatus,
    MoonSendError,
};

pub const DEFAULT_WRITER_BUFFER_SIZE: usize = 1000;
pub const DEFAULT_READER_BUFFER_SIZE: usize = 1000;
const DEFAULT_MOONRAKER_PORT: u16 = 7125;

const PRINTER_READY_TIMEOUT: u64 = 240; // Timeout in seconds
const MAX_RESTARTS: u8 = 3;             // Maximum restart attempts
const DEFAULT_SEND_LISTEN_TIMEOUT: Duration = Duration::from_secs(60);


// #[derive(thiserror::Error, Debug, Clone, PartialEq)]
// pub enum MoonSendError<T> {
//     #[error("Error sending message: {0}")]
//     SendError(#[from] mpsc::error::SendError<T>),
//     #[error("Moonraker error: {0}")]
//     MoonError(MoonErrorContent),
//     #[error("Error: {0}")]
//     String(String),
// }

// impl<T> From<Box<dyn StdError>> for MoonSendError<T> {
//     fn from(err: Box<dyn StdError>) -> Self {
//         MoonSendError::String(err.to_string())
//     }
// }

pub struct MoonConnection {
    write_stream: mpsc::Sender<MoonRequest>,
    read_stream: mpsc::Receiver<MoonResponse>,
    shutdown_sender: mpsc::Sender<()>,  // No data needed for shutdown
    id_counter: AtomicUsize,
    pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<MoonResponse>>>>,
}
impl MoonConnection {
    pub async fn new_simple(hostname: String, port: Option<u16>, debug: bool) -> Result<MoonConnection, Box<dyn std::error::Error>> {
        let port = port.unwrap_or(DEFAULT_MOONRAKER_PORT);
        let url = format!("ws://{hostname}:{port}/websocket");
        Self::new(url, None, None, debug).await
    }
    /// Creates a new `MoonConnection` instance and establishes a WebSocket connection to the specified `url`.
    ///
    /// # Arguments
    ///
    /// * `url` - A `String` containing the URL of the Moonraker instance to connect to.
    /// * `writer_buffer_size` - The size of the buffer used to store outgoing messages.
    /// * `reader_buffer_size` - The size of the buffer used to store incoming messages.
    ///
    /// # Returns
    ///
    /// A new `MoonConnection` instance.
    // pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> MoonConnection {
    pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Result<MoonConnection, Box<dyn std::error::Error>> {
        let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
        let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);

        let (ws_writer_sender, mut ws_writer_receiver) = mpsc::channel(writer_buffer_size);
        let (shutdown_sender, mut shutdown_receiver) = mpsc::channel(10);

        let connect_addr = Url::parse(&url).unwrap();

        let (ws_stream, _) = connect_async(&connect_addr)
            .await
            .map_err(|e| format!("Error connecting to websocket: {}", e))?;

        if debug {
            println!("WebSocket handshake has been successfully completed");
        }

        let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
        let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

        // Spawns Moonraker Websocket writer thread
        tokio::spawn(async move {
            while let Some(msg) = ws_writer_receiver.recv().await {
                match websocket_shutdown_receiver.try_recv() {
                    Ok(()) => {
                        break;
                    },
                    Err(_) => {},
                }

                match serde_json::to_vec(&msg) {
                    Ok(mut vec) => {
                        vec.truncate(vec.len());

                        match Pin::new(&mut moon_socket_sink).send(Message::binary(vec)).await {
                            Ok(_) => {},
                            Err(e) => eprintln!("Error sending to moon_socket_sink: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Error serializing request: {}", e),
                }
            }
        });

        let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
        
        let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<MoonResponse>>::new()));
        let pending_requests_clone = Arc::clone(&pending_requests);
        // Spawns Moonraker Websocket reader thread
        tokio::spawn(async move {
            while let Some(message) = moon_socket_stream.next().await {
                match shutdown_receiver.try_recv() {
                    Ok(()) => {
                        match websocket_shutdown_sender.send(()).await {
                            Ok(_) => {},
                            Err(e) => {
                                println!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
                            },
                        }
                        break;
                    },
                    Err(_) => {},
                }
                match message {
                    Ok(msg) => {
                        if msg.len() == 0 {
                            continue;
                        }
                        let message = msg.into_text().unwrap();
                        if debug {
                            println!("Received: {}", message);
                        }
                        let parsed = serde_json::from_str(&message);
                        match parsed {
                            Ok(message) => {
                                match message {
                                    MoonResponse::MoonResult { id, .. } | MoonResponse::MoonError { id: Some(id), .. } => {
                                        if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
                                            tx.send(message).ok();
                                        } else {
                                            ws_reader_sender.send(message).await.ok();
                                        }
                                    },
                                    _ => {},
                                }
                            },
                            Err(_) => {
                                println!("----------------------------MESSAGE NOT PARSED----------------------------");
                                println!("Message Length: {}", message.len());
                                println!("{}", message);
                                println!("--------------------------------------------------------------------------");
                            },
                        }
                    }
                    Err(e) => eprintln!("Error message from moonraker socket: {}", e.to_string()),
                }
            }
        });

        Ok(MoonConnection {
            write_stream: ws_writer_sender,
            read_stream: ws_reader_receiver,
            shutdown_sender,
            id_counter: AtomicUsize::new(1),
            pending_requests,
        })
    }

    // pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    //     match self.shutdown_sender.send(()).await {
    //         Ok(_) => {
    //             Ok(())
    //         },
    //         Err(e) => {
    //             Err(format!("Failed to send shutdown signal to the moonraker message stream: {}", e.to_string()).into())
    //         },
    //     }
    // }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.shutdown_sender.send(()).await?;  // Send empty value
        Ok(())
    }

    // pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    //     // Send a close message to gracefully shutdown the websocket connection
    //     if let Err(e) = self.moon_socket_sink.send(Message::Close(None)).await {
    //         eprintln!("Error sending close message: {}", e);
    //     }
        
    //     // Signal the tasks to shutdown
    //     self.shutdown_sender.send(()).await?;
    //     Ok(())
    // }
    
    /// The most basic of the sender methods. Adds a message to the WebSocket writer queue to be sent to the Moonraker instance.
    /// This function is non-blocking and returns immediately.
    /// When the websocket writer queue is full, this function will not block, but will yield back to the parent function until a spot opens up in the queue which
    /// can be probelmatic if the order of messages you're sending to the printer is important. In that case, consider using the `send_reserved` function instead. 
    ///
    /// # Arguments
    ///
    /// * `message` - The `MoonMSG` message to send.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the message was successfully added to the queue, or a `SendError<MoonMSG>` if the queue is full.
    // pub async fn send(&mut self, message: MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
    //     match self.write_stream.send(message).await {
    //         Ok(()) => Ok(()),
    //         Err(e) => Err(format!("Error sending `{:?}` to the `MoonConnection` request channel: {}", e.0, e.to_string()).into())
    //     }
    // }

    pub async fn send(&mut self, message: MoonRequest) -> Result<(), MoonSendError<MoonRequest>> {
        self.write_stream.send(message).await?;
        Ok(())
    }
    
    /// Sends a message over the WebSocket connection, using a reserved spot in the writer queue.
    ///
    /// This function is similar to `send`, but it uses a reserved spot in the writer queue, so it
    /// doesn't block the thread if the queue is full, but also ensures that the order of messages is exactly as your program describes. 
    /// Use this function if you are sending a lot of messages in a short amount of time and the order of those messages matters.
    /// 
    /// Essentially, you are putting a dynamic buffer on top of the fixed-sized primary message buffer to ensure that messages are sent in the order you want.
    /// 
    /// For example, say you wanted to ensure the printer recieved a `G28` (Home Printer) command before a `G1 Z10` (move printer bed up) command. 
    /// Although, in this case, you'll want to sleep for a bit after the `G28` command to ensure the printer has time to home before moving the bed up.
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send over the WebSocket connection.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the message was successfully sent or not. An error here indicates that the websocket channel is probably closed.
    // pub async fn send_reserved(&mut self, mut message: MoonRequest) -> Result<(), MoonSendError<()>> {
    pub async fn send_reserved(&mut self, mut message: MoonRequest) -> Result<(), MoonSendError<()>> {
        let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
        message.id = this_id;
        let permit = self.reserve().await.map_err(|e| MoonSendError::SendError(e))?;
        permit.send(message);
        Ok(())
    }
    
    /// Reserves a permit from the WebSocket writer queue for sending a message.
    ///
    /// # Returns
    ///
    /// Returns a `Permit<MoonMSG>` if a permit was successfully reserved, or a `SendError<()>` if the connection has closed.
    pub async fn reserve(&self) -> Result<Permit<MoonRequest>, SendError<()>> {
        self.write_stream.reserve().await
    }

    /// Waits for a message to be received from the Moonraker instance.
    ///
    /// # Returns
    ///
    /// Returns an `Option<MoonMSG>` containing the received message, or `None` if the receiver channel has been closed.
    pub async fn recv(&mut self) -> Option<MoonResponse> {
        self.read_stream.recv().await
    }

    pub async fn send_listen(&mut self, mut message: MoonRequest) -> Result<MoonResponse, MoonSendError<MoonRequest>> {
        let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
        message.id = this_id;

        let (tx, rx) = oneshot::channel();
        self.pending_requests.lock().await.insert(this_id, tx);

        self.send(message).await?; // Send the message

        // Await the response on the dedicated channel
        // match rx.await {
        //     Ok(response) => match response {
        //         MoonResponse::MoonResult {..} 
        //         | MoonResponse::Notification {..} => Ok(response),
        //         MoonResponse::MoonError {error, ..} => Err(MoonSendError::MoonError(error)),
        //     }
        //     Err(e) => Err(MoonSendError::String(format!("Oneshot channel error: {}", e.to_string()))),
        // }

        match tokio::time::timeout(DEFAULT_SEND_LISTEN_TIMEOUT, rx).await { // Example timeout of 5 seconds
            Ok(Ok(response)) => match response {
                MoonResponse::MoonResult {..} 
                | MoonResponse::Notification {..} => Ok(response),
                MoonResponse::MoonError {error, ..} => Err(MoonSendError::MoonError(error)),
            },
            Ok(Err(_)) => Err(MoonSendError::String("Channel closed unexpectedly".to_string())),
            Err(_) => Err(MoonSendError::String("Timeout waiting for response".to_string())),
        }
    }
    
    pub async fn send_wait_for_ok(&mut self, message: MoonRequest) -> Result<(), MoonSendError<MoonRequest>> {
        let res = self.send_listen(message).await?;
        match res {
            MoonResponse::MoonResult { result: MoonResultData::Ok(..), .. } => Ok(()),
            // Should never be possible to get the `MoonResponse::MoonError` variant as long as the logic of `send_listen` never changes, but
            // for correctness reasons we should still check for it.
            MoonResponse::MoonError { error, .. } => Err(MoonSendError::MoonError(error)),
            _ => Err(MoonSendError::String(format!("Expected an Ok response got: {res:?}"))),
        }
    }

    pub async fn ensure_ready(&mut self) -> PrinterSafetyStatus {
        let mut sp = Spinner::new(spinners::Dots, "Loading...", Color::Blue);

        let start_time = Instant::now();
        let mut restart_count = 0;

        loop {
            if start_time.elapsed() > Duration::from_secs(PRINTER_READY_TIMEOUT) {
                sp.stop();
                return PrinterSafetyStatus::TimeoutReached;
            }

            let printer_state = match self.get_server_info().await {
                Ok(info) => info.klippy_state,
                Err(err) => {
                    sp.stop();
                    return PrinterSafetyStatus::OtherError(err);
                }
            };

            match printer_state {
                PrinterState::Ready | PrinterState::Standby => {
                    sp.stop_with_message("Printer is ready!".into());
                    return PrinterSafetyStatus::Ready;
                },
                PrinterState::Startup => {
                    sp.update(spinners::Dots9, "Printer is starting up..", None);
                    sleep(Duration::from_millis(500)).await; // Shorter wait with spinner
                    continue;
                },
                PrinterState::Paused | PrinterState::Printing | PrinterState::Complete | PrinterState::Cancelled => {
                    sp.stop_with_message(format!("Printer is in {:?} state. Please clear the print bed.", printer_state).as_str());
                    return PrinterSafetyStatus::Maybe3DPrintInsidePrinter(printer_state);
                },
                PrinterState::Error => {
                    let error_message = match self.get_server_info().await {
                        Ok(info) => info.warnings.join(", "),
                        Err(_) => "Unknown error".to_string(),
                    };
                    if restart_count >= MAX_RESTARTS {
                        sp.stop_and_persist("❌", "Too many firmware restarts");
                        return PrinterSafetyStatus::TooManyRestarts;
                    }

                    sp.stop();
                    println!("{}", error_message);
                    if self.prompt_for_restart().await.unwrap() {
                        sp = Spinner::new(spinners::Dots9, "Running firmware restart...", Color::Blue);
                        self.firmware_restart().await.ok(); // Ignore errors during restart
                        restart_count += 1;
                    } else {
                        return PrinterSafetyStatus::KlipperError(error_message); 
                    }
                },
                PrinterState::Shutdown => {
                    if restart_count >= MAX_RESTARTS {
                        sp.stop_and_persist("❌", "Too many firmware restarts");
                        return PrinterSafetyStatus::TooManyRestarts;
                    }
                    sp.stop();
                    if self.prompt_for_restart().await.unwrap() {
                        sp = Spinner::new(spinners::Dots9, "Running firmware restart...", Color::Blue);
                        self.firmware_restart().await.ok(); // Ignore errors during restart
                        restart_count += 1;
                    } else {
                        return PrinterSafetyStatus::Shutdown; 
                    }
                },
                PrinterState::Disconnected => {
                    sp.update(spinners::Dots9, "Printer is disconnected", None);
                }
            }
        }
    }

    async fn prompt_for_restart(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let mut stdout = stdout();
        stdout.write_all(b"Do you want to attempt a firmware restart? (y/n): ").await?;
        stdout.flush().await?;

        let stdin = stdin();
        let mut reader = FramedRead::new(stdin, LinesCodec::new());
        let input = reader.next().await.transpose()?.unwrap();
        Ok(input.trim().to_lowercase() == "y")
    }

    async fn firmware_restart(&mut self) -> Result<(), Box<dyn StdError>> {
        let message = MoonRequest::new(MoonMethod::PrinterFirmwareRestart, None);
        self.send(message).await?;
        Ok(())
    }

    /// Checks if the printer is in the "Ready" state.
    pub async fn is_printer_ready(&mut self) -> Result<bool, Box<dyn StdError>> {
        let server_info = self.get_server_info().await?;
        Ok(server_info.klippy_state == PrinterState::Ready)
    }

    pub async fn get_server_info(&mut self) -> Result<ServerInfo, Box<dyn std::error::Error>> {
        let message = MoonRequest::new(MoonMethod::ServerInfo, None);
        let res = self.send_listen(message).await?;
        match res {
            MoonResponse::MoonResult { result, .. } => {
                match result {
                    MoonResultData::ServerInfo(server_info) => {
                        Ok(server_info)
                    },
                    _ => {
                        Err(format!("Unexpected response: {:?}", result).into())
                    },
                }
            },
            _ => {
                Err(format!("Unexpected response: {:?}", res).into())
            },
        }
    }

    pub async fn get_printer_info(&mut self, message_id: Option<u32>) -> Result<PrinterInfoResponse, Box<dyn std::error::Error>> {
        let message = MoonRequest::new(MoonMethod::PrinterInfo, None);
        let res = self.send_listen(message).await?;
        match res {
            MoonResponse::MoonResult { result, id, .. } => {
                match message_id {
                    Some(msg_id) => {
                        if msg_id != id {
                            println!("IDs of request and response do no match in method `get_printer_info`");
                        }
                    },
                    None => {},
                }

                match result {
                    MoonResultData::Ok(_) => Err("Recived an ok() response from the server, but was expecting ".into()),
                    MoonResultData::PrinterInfoResponse(printer_info) => {
                        return Ok(printer_info);
                    },
                    _ => Err("Error in `MoonConnection::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.".into())
                }
            },
            _ => Err("Error in `MoonConnection::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.".into()),
        }
    }

    pub async fn get_homed_axes(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let param = MoonParam::PrinterObjectsQuery{
            objects: PrinterObject::Toolhead(Some(vec!["homed_axes".to_string()])),
        };
        let msg = MoonRequest::new(MoonMethod::PrinterObjectsQuery, Some(param));

        match self.send_listen(msg).await? {
            MoonResponse::MoonResult { result, .. } => {
                match result {
                    MoonResultData::PrinterObjectsQueryResponse(res) => {
                        match res.status.toolhead {
                            Some(toolhead) => {
                                match toolhead.homed_axes {
                                    Some(homed_axes) => Ok(homed_axes),
                                    None => Err("Error: Could not find `homed_axes` in response from printer".into()),
                                }
                            },
                            None => Err("Error: Could not find the `toolhead` field in response from printer".into()),
                        }
                    },
                    _ => Err("Error: Printer did not return expected response".into()),
                }
            },
            _ => Err("Error: Printer did not return expected response".into()),
        }
    }

    pub async fn is_homed(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let homed_axes = self.get_homed_axes().await?;
        // Assuming 'XYZ' or similar indicates all required axes are homed 
        Ok(homed_axes.to_lowercase().contains("xyz")) 
    }
    
    pub async fn is_z_tilt_applied(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let param = MoonParam::PrinterObjectsQuery {
            objects: PrinterObject::ZTilt(None), 
        };
        let msg = MoonRequest::new(MoonMethod::PrinterObjectsQuery, Some(param));

        match self.send_listen(msg).await? {
            MoonResponse::MoonResult { result, .. } => match result {
                MoonResultData::PrinterObjectsQueryResponse(res) => {
                    match res.status.z_tilt {
                        Some(z_tilt) => Ok(z_tilt.applied),
                        None => Err("Error: 'z_tilt' object not found in response".into()),
                    }
                }
                _ => Err("Error: Unexpected response format from Moonraker".into()),
            },
            _ => Err("Error: Unexpected response type from Moonraker".into()),
        }
    }
}