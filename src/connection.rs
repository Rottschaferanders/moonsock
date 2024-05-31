// use std::ops::FromResidual;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
// use tokio::time::sleep;
use std::collections::HashMap;
use std::error::Error as StdError;

use std::time::{Duration, Instant};
use tokio::{
    time::sleep,
    io::{
        AsyncWriteExt, 
        stdin, stdout,
    },
};
use tokio_util::codec::{FramedRead, LinesCodec};
// use futures::StreamExt;
// use std::io::{stdin, stdout, Write};
// use spinners::{Spinner, Spinners}; // Import spinner functionality
use spinoff::{Spinner, spinners, Color};
// use std::thread::sleep;

use tokio::sync::mpsc::{
    error::SendError,
    Permit,
};
use tokio::sync::{mpsc, oneshot};

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use core::pin::Pin;
use futures_util::{sink::*, StreamExt};
use url::Url;
// use crate::*;

use crate::response::{PrinterState, ServerInfo};
use crate::{
    // moon_param::PrinterObject, 
    // MoonMSG, 
    response::{
        MoonResultData, 
        // PrinterObjectStatus, Toolhead, ZTilt
    }, 
    MoonErrorContent, MoonMethod, MoonParam, MoonRequest, MoonResponse, 
    PrinterInfoResponse, 
    PrinterObject,
};

pub const DEFAULT_WRITER_BUFFER_SIZE: usize = 1000;
pub const DEFAULT_READER_BUFFER_SIZE: usize = 1000;
const DEFAULT_MOONRAKER_PORT: u16 = 7125;

const PRINTER_READY_TIMEOUT: u64 = 240; // Timeout in seconds
const MAX_RESTARTS: u8 = 3;             // Maximum restart attempts

#[derive(Debug, Clone, PartialEq)]
pub enum MoonSendError<T> {
    SendError(tokio::sync::mpsc::error::SendError<T>),
    MoonError(MoonErrorContent),
    MoonResult(MoonResultData),
    String(String),
}

// impl<T> Into<tokio::sync::mpsc::error::SendError<T>> for MoonSendError<T> {
impl<T> Into<MoonSendError<T>> for tokio::sync::mpsc::error::SendError<T> {
    fn into(self) -> MoonSendError<T> {
        MoonSendError::SendError(self)
    }
}

// Implement From for MoonSendError to handle Box<dyn Error>
impl<T> From<Box<dyn StdError>> for MoonSendError<T> {
    fn from(err: Box<dyn StdError>) -> Self {
        // You can customize this further, e.g., extracting error details
        MoonSendError::String(err.to_string())
    }
}

// impl<T> FromResidual<Result<Infallible, Box<dyn Error>>> for MoonSendError<T> {
//     fn from_residual(residual: Result<Infallible, Box<dyn Error>>) -> Self {
//         let err = residual.unwrap_err();
//         MoonSendError::String(err.to_string())
//     }
// }

// impl<T> From<Box<dyn std::error::Error>> for MoonSendError<T> {
//     fn from(e: Box<dyn std::error::Error>) -> MoonSendError<T> {
//         MoonSendError::String(e.to_string())
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum MoonSendError<T> {
//     SendError(tokio::sync::mpsc::error::SendError<T>),
//     MoonError(MoonErrorContent),
//     MoonResult(MoonResultData),
// }

// impl<T> Into<MoonSendError<T>> for tokio::sync::mpsc::error::SendError<T> {
//     fn into(self) -> MoonSendError<T> {
//         MoonSendError::SendError(self)
//     }
// }

// impl<T> From<Box<dyn StdError>> for MoonSendError<T> {
//     fn from(err: Box<dyn StdError>) -> Self {
//         MoonSendError::MoonError(MoonErrorContent {
//             code: 0,
//             message: err.to_string(),
//         })
//     }
// }

impl<T> std::fmt::Display for MoonSendError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MoonSendError::SendError(err) => write!(f, "Send error: {}", err),
            MoonSendError::MoonError(err) => write!(f, "Moon error: {}", err),
            MoonSendError::MoonResult(err) => write!(f, "Moon result error: {}", err),
            MoonSendError::String(err) => write!(f, "String error: {}", err),
        }
    }
}

pub enum PrinterSafetyStatus {
    Ready,
    Maybe3DPrintInsidePrinter(PrinterState),
    KlipperError(String),
    Shutdown,
    TimeoutReached,
    TooManyRestarts,
    OtherError(Box<dyn std::error::Error>),
}

/// A WebSocket connection to a Moonraker server.
pub struct MoonConnection {
    write_stream: tokio::sync::mpsc::Sender<MoonRequest>,
    read_stream: tokio::sync::mpsc::Receiver<MoonResponse>,
    shutdown_sender: tokio::sync::mpsc::Sender<bool>,
    id_counter: AtomicUsize,
    pending_requests: Arc<Mutex<HashMap<u32, mpsc::Sender<MoonResponse>>>>,
}
impl MoonConnection {
    pub async fn new_simple(hostname: String, port: Option<u16>, debug: bool) -> MoonConnection {
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
    pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> MoonConnection {
        let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
        let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);
        let (ws_writer_sender, mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
        let (shutdown_sender, mut shutdown_receiver) = tokio::sync::mpsc::channel(10);

        let connect_addr = Url::parse(&url).unwrap();
        let (ws_stream, _) = match connect_async(&connect_addr).await {
            Ok(stuff) => stuff,
            Err(_) => panic!("Error connecting to websocket"),
        };
        if debug {
            println!("WebSocket handshake has been successfully completed");
        }

        let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
        let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

        // Spawns Moonraker Websocket writer thread
        tokio::spawn(async move {
            while let Some(msg) = ws_writer_receiver.recv().await {
                // I think we want the websocket spawned loop to recieve the shutdown signal from 
                // the MoonMSG parsing stream.
                match websocket_shutdown_receiver.try_recv() {
                    Ok(should_shutdown) => {
                        if should_shutdown {
                            break;
                        }
                    },
                    Err(_) => {},
                }
                let mut vec = serde_json::to_vec(&msg).expect("Could not Serialize Request");
                vec.truncate(vec.len());
                let result = Pin::new(&mut moon_socket_sink)
                    .send(Message::binary(vec))
                    .await;
                match result {
                    Ok(_) => continue,
                    Err(_) => println!("Unable to send to moon_socket_sink"),
                }
            }
        });

        let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
        
        let pending_requests = Arc::new(Mutex::new(HashMap::<u32, mpsc::Sender<MoonResponse>>::new()));
        let pending_requests_clone = Arc::clone(&pending_requests);
        // Spawns Moonraker Websocket reader thread
        tokio::spawn(async move {
            while let Some(message) = moon_socket_stream.next().await {
                // Check if we've received a shutdown signal and leave the while loop if so
                match shutdown_receiver.try_recv() {
                    Ok(should_shutdown) => {
                        if should_shutdown {
                            match websocket_shutdown_sender.send(true).await {
                                Ok(_) => {
                                    break;
                                },
                                Err(e) => {
                                    println!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
                                },
                            }
                            break;
                        }
                    },
                    Err(_) => {},
                }
                match message {
                    Ok(msg) => {
                        // if msg.len() == 0 {
                        //     continue;
                        // }
                        // let message = msg.into_text().unwrap();
                        // if debug {
                        //     println!("Received: {}", message);
                        // }
                        // let parsed = serde_json::from_str(&message);
                        // match parsed {
                        //     Ok(message) => match ws_reader_sender.send(message).await {
                        //         Ok(()) => continue,
                        //         Err(_) => println!("Unable to send to ws_reader_sender"),
                        //     },
                        //     Err(_) => {
                        //         println!("----------------------------MESSAGE NOT PARSED----------------------------");
                        //         println!("Message Length: {}", message.len());
                        //         println!("{}", message);
                        //         println!("--------------------------------------------------------------------------");
                        //     },
                        // }


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
                                            tx.send(message).await.ok();
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
                    Err(e) => {
                        eprintln!("Error message from moonraker socket: {}", e.to_string());
                    },
                }
            }
        });

        MoonConnection {
            write_stream: ws_writer_sender,
            read_stream: ws_reader_receiver,
            shutdown_sender,
            id_counter: AtomicUsize::new(1),
            pending_requests,
        }
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.shutdown_sender.send(true).await {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                // eprintln!("Failed to send shutdown signal to the moonraker message stream");
                Err(format!("Failed to send shutdown signal to the moonraker message stream: {}", e.to_string()).into())
            },
        }
    }
    /// Creates a new `MoonConnection` instance and establishes a WebSocket connection to the specified `url`.
    /// The only difference between this and `new` is that this function println!s out all received messages.
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
    // pub async fn new_debug(url: String, writer_buffer_size: usize, reader_buffer_size: usize) -> MoonConnection {
    //     let (ws_writer_sender, mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
    //     let connect_addr = Url::parse(&url).unwrap();
    //     let (ws_stream, _) = match connect_async(&connect_addr).await {
    //         Ok(stuff) => stuff,
    //         Err(_) => panic!("Error connecting to websocket"),
    //     };
    //     println!("WebSocket handshake has been successfully completed");
    //     let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

    //     // Spawns Moonraker Websocket writer thread
    //     tokio::spawn(async move {
    //         while let Some(msg) = ws_writer_receiver.recv().await {
    //             let mut vec = serde_json::to_vec(&msg).expect("Could not Serialize Request");
    //             vec.truncate(vec.len());
    //             let result = Pin::new(&mut moon_socket_sink)
    //                 .send(Message::binary(vec))
    //                 .await;
    //             match result {
    //                 Ok(_) => continue,
    //                 Err(_) => println!("Unable to send to moon_socket_sink"),
    //             }
    //         }
    //     });
    //     println!("Split websocket Stream");
    //     let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
    //     println!("Created Reader Runtime");
    //     // Spawns Moonraker Websocket reader thread
    //     tokio::spawn(async move {
    //         while let Some(message) = moon_socket_stream.next().await {
    //             match message {
    //                 Ok(msg) => {
    //                     if msg.len() == 0 {
    //                         continue;
    //                     }
    //                     let message = msg.into_text().unwrap();
    //                     println!("Received: {}", message);
    //                     let parsed = serde_json::from_str(&message);
    //                     match parsed {
    //                         Ok(message) => match ws_reader_sender.send(message).await {
    //                             Ok(()) => continue,
    //                             Err(_) => println!("Unable to send to ws_reader_sender"),
    //                         },
    //                         Err(_) => {
    //                             println!("----------------------------MESSAGE NOT PARSED----------------------------");
    //                             println!("Message Length: {}", message.len());
    //                             println!("{}", message);
    //                             println!("--------------------------------------------------------------------------");
    //                         },
    //                     }
    //                 }
    //                 Err(_) => println!("Hi, I'm a error"),
    //             }
    //         }
    //     });

    //     MoonConnection {
    //         write_stream: ws_writer_sender,
    //         read_stream: ws_reader_receiver,
    //     }
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
    // pub async fn send(&mut self, message: MoonMSG) -> Result<(), SendError<MoonMSG>> {
    // pub async fn send(&mut self, message: MoonRequest) -> Result<(), SendError<MoonResponse>> {
    // pub async fn send(&mut self, message: MoonRequest) -> Result<(), MoonSendError<MoonRequest>> {
    pub async fn send(&mut self, message: MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
        match self.write_stream.send(message).await {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Error sending `{:?}` to the `MoonConnection` request channel: {}", e.0, e.to_string()).into())
        }
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
    // pub async fn send_reserved(&mut self, message: MoonMSG) -> Result<(), SendError<()>> {
    // pub async fn send_reserved(&mut self, message: MoonRequest) -> Result<(), SendError<()>> {
    // pub async fn send_reserved(&mut self, message: MoonRequest) -> Result<(), MoonSendError<()>> {
    //     let permit = self.reserve().await.map_err(|e| e.into())?;
    //     permit.send(message);
    //     Ok(())
    // }
    // pub async fn send_reserved(&mut self, mut message: MoonRequest) -> Result<(), MoonSendError<()>> {
    // pub async fn send_reserved(&mut self, mut message: MoonRequest) -> Result<(), MoonSendError<()>> {
    //     let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
    //     message.id = this_id;
    //     let permit = self.reserve().await.map_err(|e| e.into())?;
    //     permit.send(message);
    //     Ok(())
    // }
    pub async fn send_reserved(&mut self, mut message: MoonRequest) -> Result<(), MoonSendError<()>> {
        let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
        message.id = this_id;
        // let permit = self.reserve().await.map_err(|e| e.into())?;
        let permit = self.reserve().await.map_err(|e| MoonSendError::SendError(e))?;
        permit.send(message);
        Ok(())
    }
    
    /// Reserves a permit from the WebSocket writer queue for sending a message.
    ///
    /// # Returns
    ///
    /// Returns a `Permit<MoonMSG>` if a permit was successfully reserved, or a `SendError<()>` if the connection has closed.
    // pub async fn reserve(&self) -> Result<Permit<MoonMSG>, SendError<()>> {
    pub async fn reserve(&self) -> Result<Permit<MoonRequest>, SendError<()>> {
        self.write_stream.reserve().await
    }
    /// Waits for a message to be received from the Moonraker instance.
    ///
    /// # Returns
    ///
    /// Returns an `Option<MoonMSG>` containing the received message, or `None` if the receiver channel has been closed.
    // pub async fn recv(&mut self) -> Option<MoonMSG> {
    pub async fn recv(&mut self) -> Option<MoonResponse> {
        self.read_stream.recv().await
    }
    /// Sends message and then waits for the printer to send an Ok message back
    // pub async fn send_checked(&mut self, message: MoonMSG) -> Result<(), SendError<MoonMSG>> {
    // pub async fn send_wait_for_ok(&mut self, message: MoonMSG) -> Result<(), SendError<MoonMSG>> {
    // pub async fn send_wait_for_ok(&mut self, message: MoonRequest) -> Result<(), SendError<MoonResponse>> {
    // pub async fn send_wait_for_ok(&mut self, message: MoonRequest) -> Result<(), MoonSendError<MoonRequest>> {
    // pub async fn send_wait_for_ok(&mut self, mut message: MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
    //     // let this_id = 3243;
    //     // Should probably keep a vector of ids that have already been used so that
    //     // we can check against it for collisions. Would incure a lot of performance cost
    //     // especially in some situations where many commands are sent to the printer in a short amount of time.
    //     // let this_id = rand::random();
    //     // let msg = message.set_id(this_id);
    //     // let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
    //     // message.id = this_id;
    //     let id = self.id_counter.fetch_add(1, Ordering::SeqCst);
    //     message.id = id;
    //     // let this_id = message.id;
    //     self.send(message).await?;
    //     loop {
    //         match self.recv().await {
    //             Some(msg) => {
    //                 match msg {
    //                     MoonResponse::MoonResult { id, result, .. } => {
    //                         if id == this_id {
    //                             match result {
    //                                 MoonResultData::Ok(..) => {
    //                                     return Ok(());
    //                                 },
    //                                 _ => continue,
    //                             }
    //                         }
    //                     },
    //                     _ => continue,
    //                 }
    //             },
    //             None => continue,
    //         }
    //     }
    // }
    // pub async fn send_wait_for_ok(&mut self, mut message: MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
    //     let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
    //     message.id = this_id;
    //     // let this_id = id;
    //     self.send(message).await?;
    //     loop {
    //         match self.recv().await {
    //             Some(msg) => {
    //                 match msg {
    //                     MoonResponse::MoonResult { id, result, .. } => {
    //                         if id == this_id {
    //                             match result {
    //                                 MoonResultData::Ok(..) => {
    //                                     return Ok(());
    //                                 },
    //                                 _ => continue,
    //                             }
    //                         }
    //                     },
    //                     _ => continue,
    //                 }
    //             },
    //             None => continue,
    //         }
    //     }
    // }

    // pub async fn send_wait_for_ok(&mut self, mut message: MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
    //     let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
    //     message.id = this_id;

    //     let (tx, rx) = oneshot::channel();
    //     self.pending_requests.insert(this_id, tx);

    //     self.send(message).await?;

    //     match rx.await {
    //         Ok(res) => {
    //             match res {
    //                 MoonResponse::MoonResult { result, .. } => {
    //                     match result {
    //                         MoonResultData::Ok(..) => {
    //                             return Ok(());
    //                         },
    //                         _ => return Err("MoonResultData is not Ok".into()),
    //                     }
    //                 },
    //                 _ => return Err("MoonResponse is not MoonResult".into()),
    //             }
    //         },
    //         Err(_) => return Err("Failed to receive response".into()),
    //     }
    // }

    // pub async fn send_wait_for_ok(&mut self, mut message: MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
    pub async fn send_wait_for_ok(&mut self, mut message: MoonRequest) -> Result<(), MoonSendError<()>> {
        let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
        message.id = this_id;

        let (tx, mut rx) = mpsc::channel(1);
        self.pending_requests.lock().await.insert(this_id, tx);

        self.send(message).await?;

        match rx.recv().await {
            Some(MoonResponse::MoonResult { result, .. }) => {
                match result {
                    MoonResultData::Ok(..) => {
                        return Ok(());
                    },
                    _ => {
                        // return Err("Received error response".into());
                        return Err(MoonSendError::String("Received error response".to_string()));
                    },
                }
            },
            Some(MoonResponse::MoonError { error, .. }) => {
                return Err(MoonSendError::MoonError(error));
            },
            None => {
                // return Err("Channel closed".into());
                return Err(MoonSendError::String("Channel closed".to_string()));
            },
            _ => Err(MoonSendError::String("Received unexpected single shot response from message matcher".to_string())),
        }
    }
    // pub async fn send_listen(&mut self, message: MoonMSG) -> Result<MoonMSG, SendError<MoonMSG>> {
    // pub async fn send_listen(&mut self, message: MoonRequest) -> Result<MoonResponse, SendError<MoonResponse>> {
    // pub async fn send_listen(&mut self, message: MoonRequest) -> Result<MoonResponse, MoonSendError<MoonRequest>> {
    // pub async fn send_listen(&mut self, message: MoonRequest) -> Result<MoonResponse, Box<dyn std::error::Error>> {
    //     // let this_id = message.id().expect("Message must have an ID");
    //     // let this_id = message.id().unwrap_or(rand::random());
    //     let this_id = message.id;
    //     self.send(message).await?;
    //     // match self.send(message).await {
    //     //     Ok(()) => {},
    //     //     Err(e) => Err(format!(""))
    //     // }
    //     loop {
    //         match self.recv().await {
    //             Some(res) => {
    //                 match res {
    //                     MoonResponse::MoonResult { result, id, .. } => {
    //                         if id == this_id {
    //                             // return Ok(res.clone());
    //                             return Ok(MoonResponse::MoonResult { jsonrpc: crate::JsonRpcVersion::V2, result, id });
    //                         }
    //                     },
    //                     MoonResponse::MoonError { error, id, .. } => {
    //                         match id {
    //                             Some(id) => {
    //                                 if id == this_id {
    //                                     // return Ok(res.clone())
    //                                     return Ok(MoonResponse::MoonError { jsonrpc: crate::JsonRpcVersion::V2, error, id: Some(id) });
    //                                 }
    //                             },
    //                             None => continue,
    //                         }
    //                     },
    //                     MoonResponse::Notification { .. } => {},
    //                 }
    //                 // match msg.id {
    //                 //     Some(id) => {
    //                 //         if id == this_id {
    //                 //             return Ok(msg)
    //                 //         }
    //                 //     },
    //                 //     None => continue,
    //                 // }
    //             },
    //             None => continue,
    //         }
    //     }
    // }

    // pub async fn send_listen(&mut self, mut message: MoonRequest) -> Result<MoonResponse, Box<dyn std::error::Error>> {
    //     let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
    //     message.id = this_id;
    //     // let this_id = id;
    //     self.send(message).await?;
    //     loop {
    //         match self.recv().await {
    //             Some(res) => {
    //                 match res {
    //                     MoonResponse::MoonResult { result, id, .. } => {
    //                         if id == this_id {
    //                             return Ok(MoonResponse::MoonResult { jsonrpc: crate::JsonRpcVersion::V2, result, id });
    //                         }
    //                     },
    //                     MoonResponse::MoonError { error, id, .. } => {
    //                         match id {
    //                             Some(id) => {
    //                                 if id == this_id {
    //                                     return Ok(MoonResponse::MoonError { jsonrpc: crate::JsonRpcVersion::V2, error, id: Some(id) });
    //                                 }
    //                             },
    //                             None => continue,
    //                         }
    //                     },
    //                     MoonResponse::Notification { .. } => {},
    //                 }
    //             },
    //             None => continue,
    //         }
    //     }
    // }
    pub async fn send_listen(&mut self, mut message: MoonRequest) -> Result<MoonResponse, Box<dyn std::error::Error>> {
        let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
        message.id = this_id;
        self.send(message).await?;
        loop {
            match self.recv().await {
                Some(res) => {
                    match res {
                        MoonResponse::MoonResult { result, id, .. } => {
                            if id == this_id {
                                return Ok(MoonResponse::MoonResult { jsonrpc: crate::JsonRpcVersion::V2, result, id });
                            }
                        },
                        MoonResponse::MoonError { error, id, .. } => {
                            if let Some(id) = id {
                                if id == this_id {
                                    return Ok(MoonResponse::MoonError { jsonrpc: crate::JsonRpcVersion::V2, error, id: Some(id) });
                                }
                            }
                        },
                        MoonResponse::Notification { .. } => {},
                    }
                },
                None => continue,
            }
        }
    }
    // pub async fn send_listen_debug(&mut self, message: MoonMSG) -> Result<MoonMSG, SendError<MoonMSG>> {
    // pub async fn send_listen_debug(&mut self, message: MoonRequest) -> Result<MoonResponse, MoonSendError<MoonRequest>> {
    //     // let this_id = message.id().unwrap_or(rand::random());
    //     let this_id = message.id;
    //     println!("Using message id: {this_id}");
    //     self.send(message).await?;
    //     loop {
    //         match self.recv().await {
    //             Some(msg) => {
    //                 match msg.id {
    //                     Some(id) => {
    //                         if id == this_id {
    //                             println!("Received: \n {msg:?}");
    //                             return Ok(msg)
    //                         }
    //                     },
    //                     None => continue,
    //                 }
    //             },
    //             None => continue,
    //         }
    //     }
    // }

    // pub async fn ensure_ready(&mut self) -> PrinterSafetyStatus {
    //     let start_time = Instant::now();
    //     let mut restart_count = 0;

    //     loop {
    //         if start_time.elapsed() > Duration::from_secs(PRINTER_READY_TIMEOUT) {
    //             return PrinterSafetyStatus::TimeoutReached;
    //         }
            
    //         let printer_state = match self.get_server_info().await {
    //             Ok(info) => info.klippy_state,
    //             Err(err) => return PrinterSafetyStatus::OtherError(err),
    //         };

    //         match printer_state {
    //             PrinterState::Ready | PrinterState::Standby => return PrinterSafetyStatus::Ready,
    //             PrinterState::Startup => {
    //                 sleep(Duration::from_secs(5)).await; // Wait a bit before re-checking in Startup state
    //                 continue; 
    //             },
    //             PrinterState::Paused | PrinterState::Printing | PrinterState::Complete | PrinterState::Cancelled => {
    //                 return PrinterSafetyStatus::Maybe3DPrintInsidePrinter(printer_state);
    //             },
    //             PrinterState::Error => {
    //                 let error_message = match self.get_server_info().await { // Get full error message
    //                     Ok(info) => info.warnings.join(", "), // Combine all warnings if there are multiple
    //                     Err(_) => "Unknown error".to_string(),
    //                 };
    //                 eprintln!("Printer is in Error state: {}", error_message);
    //                 if restart_count >= MAX_RESTARTS {
    //                     return PrinterSafetyStatus::TooManyRestarts;
    //                 } else if self.prompt_for_restart().await.unwrap() {
    //                     self.firmware_restart().await.ok(); // Ignore errors during restart
    //                     restart_count += 1;
    //                 } else {
    //                     return PrinterSafetyStatus::KlipperError(error_message); 
    //                 }
    //             },
    //             PrinterState::Shutdown => {
    //                 eprintln!("Printer is shutdown.");
    //                 if restart_count >= MAX_RESTARTS {
    //                     return PrinterSafetyStatus::TooManyRestarts;
    //                 } else if self.prompt_for_restart().await.unwrap() {
    //                     self.firmware_restart().await.ok(); // Ignore errors during restart
    //                     restart_count += 1;
    //                 } else {
    //                     return PrinterSafetyStatus::Shutdown; 
    //                 }
    //             },
    //         }
    //         sleep(Duration::from_secs(5)).await; // Check every 5 seconds
    //     }
    // }

    pub async fn ensure_ready(&mut self) -> PrinterSafetyStatus {
        // let mut sp = Spinner::new(Spinners::Dots9, "Connecting to printer...".into());
        let mut sp = Spinner::new(spinners::Dots, "Loading...", Color::Blue);

        let start_time = Instant::now();
        let mut restart_count = 0;

        loop {
            if start_time.elapsed() > Duration::from_secs(PRINTER_READY_TIMEOUT) {
                sp.stop(); // Stop the spinner if the timeout is reached
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
                    // sp.message("Printer is starting up...".into());
                    sp.update(spinners::Dots9, "Printer is starting up..", None);
                    // let startup_spinner = Spinner::new(Spinners::Dots9, "Printer is starting up...".into());
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
                    // println!("{:?}", error_message);
                    // sp.update(spinners::Dots9, "Printer is in Error state", None);
                    // sp.stop_and_persist("❌", "Printer is in Error state");
                    // sp.stop_with_message(format!("Printer error: {:?}", error_message).as_str());
                    if restart_count >= MAX_RESTARTS {
                        // sp.stop_with_message(format!("Printer error: {:?}", error_message).as_str());
                        sp.stop_and_persist("❌", "Too many firmware restarts");
                        // println!("{:?}", error_message);
                        return PrinterSafetyStatus::TooManyRestarts;
                    }
                    // // } else if self.prompt_for_restart().await.unwrap() {
                    // } else if self.prompt_for_restart(&mut sp).await.unwrap() {
                    //     self.firmware_restart().await.ok(); // Ignore errors during restart
                    //     restart_count += 1;
                    // } else {
                    //     sp.stop_with_message(format!("Printer error: {:?}", error_message).as_str());
                    //     return PrinterSafetyStatus::KlipperError(error_message); 
                    // }
                    sp.stop();
                    println!("{}", error_message);
                    if self.prompt_for_restart().await.unwrap() {
                        sp = Spinner::new(spinners::Dots9, "Running firmware restart...", Color::Blue);
                        self.firmware_restart().await.ok(); // Ignore errors during restart
                        restart_count += 1;
                    } else {
                        // sp.stop_with_message(format!("Printer error: {:?}", error_message).as_str());
                        return PrinterSafetyStatus::KlipperError(error_message); 
                    }

                    // sp.stop_and_persist("？", "Do you want to attempt a firmware restart? (y/n): ");

                    // let stdin = stdin();
                    // let mut reader = FramedRead::new(stdin, LinesCodec::new());
                    // let input = reader.next().await.transpose().unwrap().unwrap();
                    // if input.trim().to_lowercase() == "y" {
                    //     sp = Spinner::new(spinners::Dots9, "Running firmware restart...", Color::Blue);
                    //     // sp.update(spinners::Dots9, "Running firmware restart...", None);
                    //     self.firmware_restart().await.ok(); // Ignore errors during restart
                    //     restart_count += 1;
                    // } else {
                    //     // sp.stop_and_persist("❌", "Printer is shutdown.");
                    //     // sp.stop_and_persist("❌", "Printer has error: ");
                    //     return PrinterSafetyStatus::OtherError();
                    // }
                },
                PrinterState::Shutdown => {
                    // sp.stop_with_message("Printer is shutdown.");
                    // ... (rest of shutdown handling is the same)
                    if restart_count >= MAX_RESTARTS {
                        // sp.stop_with_message("Printer is shutdown.");
                        // sp.stop_and_persist("❌", "Too many restarts");
                        sp.stop_and_persist("❌", "Too many firmware restarts");
                        return PrinterSafetyStatus::TooManyRestarts;
                    }
                    // } else if self.prompt_for_restart().await.unwrap() {
                    // sp.stop_and_persist(, msg)
                    sp.stop();
                    if self.prompt_for_restart().await.unwrap() {
                        // sp.stop_with_message("Printer is shutdown.");
                        // sp.update(spinners::Dots9, "Running firmware restart...", None);
                        sp = Spinner::new(spinners::Dots9, "Running firmware restart...", Color::Blue);
                        self.firmware_restart().await.ok(); // Ignore errors during restart
                        restart_count += 1;
                    } else {
                        // sp.stop_with_message("Printer is shutdown.");
                        // sp.stop_and_persist("❌", "Printer is shutdown.");
                        return PrinterSafetyStatus::Shutdown; 
                    }
                    // sp.stop_and_persist("？", "Do you want to attempt a firmware restart? (y/n): ");
                    // let stdin = stdin();
                    // let mut reader = FramedRead::new(stdin, LinesCodec::new());
                    // let input = reader.next().await.transpose().unwrap().unwrap();
                    // if input.trim().to_lowercase() == "y" {
                    //     sp = Spinner::new(spinners::Dots9, "Running firmware restart...", Color::Blue);
                    //     // sp.update(spinners::Dots9, "Running firmware restart...", None);
                    //     self.firmware_restart().await.ok(); // Ignore errors during restart
                    //     restart_count += 1;
                    // } else {
                    //     // sp.stop_and_persist("❌", "Printer is shutdown.");
                    //     return PrinterSafetyStatus::Shutdown;
                    // }
                    // if self.prompt_for_restart(&mut sp).await.unwrap();
                },
                PrinterState::Disconnected => {
                    // sp.stop_with_message("Printer is disconnected.");
                    // sp.stop_and_persist("❌", "Printer is disconnected.");
                    sp.update(spinners::Dots9, "Printer is disconnected", None);
                    // return PrinterSafetyStatus::OtherError("Printer Disconnected".into());
                }
            }
        }
    }

    async fn prompt_for_restart(&self) -> Result<bool, Box<dyn std::error::Error>> {
        // let mut input = String::new();
        // print!("Do you want to attempt a firmware restart? (y/n): ");
        // stdout().flush().expect("Failed to flush stdout"); // Make sure prompt is printed
        // stdin().read_line(&mut input).expect("Failed to read input");
        // input.trim().to_lowercase() == "y" 

        let mut stdout = stdout();
        stdout.write_all(b"Do you want to attempt a firmware restart? (y/n): ").await?;
        stdout.flush().await?;

        // spinner.update(spinners::Dots9, "Do you want to attempt a firmware restart? (y/n): ", None);

        let stdin = stdin();
        let mut reader = FramedRead::new(stdin, LinesCodec::new());
        let input = reader.next().await.transpose()?.unwrap();
        Ok(input.trim().to_lowercase() == "y")
        // loop {
        //     stdout.write_all(b"Do you want to attempt a firmware restart? (y/n): ").await?;
        //     stdout.flush().await?;

        //     let stdin = io::stdin();
        //     let mut reader = FramedRead::new(stdin, LinesCodec::new());
        //     let input = reader.next().await.transpose()?.unwrap();
        //     // println!("input: {}", input);
        //     input.trim().to_lowercase() == "y"
        //     // if input.trim().to_lowercase() == "y" {
        //     //     // break; // Nozzle is clean, continue
        //     //     return Ok(true);
        //     // } else {
        //     //     stdout.write_all(b"You need to make sure it's cleaned before continuing to avoid damaging your printer.\n").await?;
        //     //     stdout.flush().await?;
        //     // }
        // }
    }

    async fn firmware_restart(&mut self) -> Result<(), Box<dyn StdError>> {
        let message = MoonRequest::new(MoonMethod::PrinterFirmwareRestart, None);
        self.send(message).await
    }


    // pub async fn ensure_ready(&mut self) -> Result<PrinterSafetyStatus, PrinterSafetyStatus> {
    //     let start_time = Instant::now();
    //     let mut restart_count = 0;
    //     println!("Ensuring printer is ready.");

    //     loop {
    //         if start_time.elapsed() > Duration::from_secs(PRINTER_READY_TIMEOUT) {
    //             println!("Timeout reached while waiting for printer to become ready.");
    //             return Err(PrinterSafetyStatus::TimeoutReached);
    //         }

    //         let server_info = self.get_server_info().await.map_err(PrinterSafetyStatus::OtherError)?;
    //         let state = server_info.klippy_state;

    //         match state {
    //             PrinterState::Ready | PrinterState::Standby => return Ok(PrinterSafetyStatus::Ready),
    //             PrinterState::Startup => {
    //                 sleep(Duration::from_secs(1)).await;
    //                 continue; // Wait a moment before checking again
    //             },
    //             PrinterState::Paused | PrinterState::Printing | PrinterState::Complete | PrinterState::Cancelled => {
    //                 return Err(PrinterSafetyStatus::Maybe3DPrintInsidePrinter(state));
    //             },
    //             PrinterState::Error => {
    //                 println!("Klipper error: {:?}", server_info.warnings); 
    //                 if !self.confirm_firmware_restart() {
    //                     return Err(PrinterSafetyStatus::KlipperError("User declined firmware restart".into()));
    //                 }
    //                 restart_count += 1;
    //                 if restart_count >= 3 {
    //                     return Err(PrinterSafetyStatus::TooManyRestarts);
    //                 }
    //             },
    //             PrinterState::Shutdown => {
    //                 println!("Printer is shutdown.");
    //                 if !self.confirm_firmware_restart() {
    //                     return Err(PrinterSafetyStatus::KlipperError("User declined firmware restart".into()));
    //                 }
    //                 restart_count += 1;
    //                 if restart_count >= 3 {
    //                     return Err(PrinterSafetyStatus::TooManyRestarts);
    //                 }
    //             },
    //         }

    //         // Restarting firmware
    //         self.firmware_restart().await;
    //         sleep(Duration::from_secs(1)).await; // Wait a moment after restart
    //     }
    // }

    // fn confirm_firmware_restart(&self) -> bool {
    //     let mut input = String::new();
    //     println!("Would you like to attempt a firmware restart? (y/n)");
    //     print!("> ");
    //     stdout().flush().unwrap();
    //     stdin().read_line(&mut input).expect("Failed to read input");
    //     input.trim().to_lowercase() == "y"
    // }

    // async fn firmware_restart(&mut self) {
    //     let message = MoonRequest::new(MoonMethod::PrinterFirmwareRestart, None);
    //     self.send(message).await.ok(); // Ignore errors for now
    // }



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
        // let message = MoonMSG::new(MoonMethod::PrinterInfo, None, message_id);
        let message = MoonRequest::new(MoonMethod::PrinterInfo, None);
        let res = self.send_listen(message).await?;
        // match self.send_listen(message).await {
            // Ok(res) => {
                // MoonMSG::MoonResult { result, id, .. } => {
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
            // },
        //     Err(e) => Err(format!("Error getting "))
        // }
    }
    pub async fn get_homed_axes(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let param = MoonParam::PrinterObjectsQuery{
            objects: PrinterObject::Toolhead(Some(vec!["homed_axes".to_string()])),
            // objects: PrinterObjectStatus {
            //     // toolhead: Toolhead(Some(vec!["homed_axes".to_string()])),
            //     toolhead: Some(Toolhead {
            //         homed_axes: Some("homed_axes".to_string()),
            //         ..Default::default()
            //     }),
            //     ..Default::default()
            // }
        };
        // let msg = MoonMSG::new(MoonMethod::PrinterObjectsQuery, Some(param), Some(1413));
        let msg = MoonRequest::new(MoonMethod::PrinterObjectsQuery, Some(param));
        // println!("Sending: {}", serde_json::to_string_pretty(&msg).unwrap());
        // let response = self.send_listen(msg).await?;

        match self.send_listen(msg).await? {
            // MoonMSG::MoonResult { result, .. } => {
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

    // pub async fn is_z_tilt_applied(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
    //     let param = MoonParam::PrinterObjectsQuery {
    //         objects: PrinterObject::ZTilt(None), 
    //     };
    //     let msg = MoonMSG::new(MoonMethod::PrinterObjectsQuery, Some(param), Some(1413)); // Example ID

    //     match self.send_listen(msg).await? {
    //         MoonMSG::MoonResult { result, .. } => match result {
    //             MoonResultData::QueryPrinterObjectsResponse(res) => {
    //                 match res.status.z_tilt {
    //                     Some(z_tilt) => Ok(z_tilt.applied),
    //                     None => Err("Error: 'z_tilt' object not found in response".into()),
    //                 }
    //             }
    //             _ => Err("Error: Unexpected response format from printer".into()),
    //         },
    //         _ => Err("Error: Unexpected response type from printer".into()),
    //     }
    // }
    
    pub async fn is_z_tilt_applied(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let param = MoonParam::PrinterObjectsQuery {
            objects: PrinterObject::ZTilt(None), 
            // objects: PrinterObjectStatus {
            //     z_tilt: Some(ZTilt {
            //         applied: trueNone)),
            //     ..Default::default()
            // } 
        };
        // let msg = MoonMSG::new(MoonMethod::PrinterObjectsQuery, Some(param), Some(1413)); // Example ID
        let msg = MoonRequest::new(MoonMethod::PrinterObjectsQuery, Some(param));

        match self.send_listen(msg).await? {
            // MoonMSG::MoonResult { result, .. } => match result {
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