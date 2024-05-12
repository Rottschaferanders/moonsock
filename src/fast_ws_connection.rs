use tokio::sync::mpsc::{
    error::SendError,
    Permit,
};
// use futures::{Stream, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use core::pin::Pin;
use core::ascii::*;
use futures_util::{sink::*, StreamExt, Stream};
use url::Url;

use fastwebsockets::{FragmentCollector, Payload};
use hyper::{Request, body::Bytes, upgrade::Upgraded, header::{UPGRADE, CONNECTION}};
use hyper_util::rt::TokioIo;

use crate::{
    fast_ws_stuff::connect, response::MoonResultData, MoonErrorContent, MoonMethod, MoonParam, MoonRequest, MoonResponse, NotificationMethod, PrinterInfoResponse, PrinterObject
};

const DEFAULT_WRITER_BUFFER_SIZE: usize = 1000;
const DEFAULT_READER_BUFFER_SIZE: usize = 1000;

pub enum MoonSendError<T> {
    SendError(tokio::sync::mpsc::error::SendError<T>),
    MoonError(MoonErrorContent),
    MoonResult(MoonResultData),
}

impl<T> Into<MoonSendError<T>> for tokio::sync::mpsc::error::SendError<T> {
    fn into(self) -> MoonSendError<T> {
        MoonSendError::SendError(self)
    }
}

/// A WebSocket connection to a Moonraker server.
pub struct FastMoonConn {
    write_stream: tokio::sync::mpsc::Sender<MoonRequest>,
    read_stream: tokio::sync::mpsc::Receiver<MoonResponse>,
    shutdown_sender: tokio::sync::mpsc::Sender<bool>,
}
impl FastMoonConn {
    pub async fn new_new(host: String, port: u16, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Self {
        let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
        let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);
        
        let (ws_writer_sender,      mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
        let (shutdown_sender,              mut shutdown_receiver) = tokio::sync::mpsc::channel(10);
        let (websocket_shutdown_sender,    mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
        // let connect_addr = Url::parse(&url).unwrap();
        let ws_stream = connect(host, port).await.unwrap();
        let mut fragment_collector = FragmentCollector::new(ws_stream);
        // let (ws_stream, _) = match connect_async(&connect_addr).await {
        //     Ok(stuff) => stuff,
        //     Err(_) => panic!("Error connecting to websocket"),
        // };

        if debug {
            println!("WebSocket handshake has been successfully completed");
        }

        // let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
        // let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

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
                // let result = Pin::new(&mut moon_socket_sink)
                //     .send(Message::binary(vec))
                //     .await;
                // match result {
                //     Ok(_) => continue,
                //     Err(_) => println!("Unable to send to moon_socket_sink"),
                // }
            }
        });

        let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);

        // Spawns Moonraker Websocket reader thread
        tokio::spawn(async move {
            loop {
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
                match fragment_collector.read_frame().await {
                    Ok(frame) => {
                        let payload = frame.payload;
                        let message = match payload {
                            Payload::BorrowedMut(contents) => {
                                // println!("BorrowedMut: {contents:?}");
                                let res_str = String::from_utf8(contents.to_vec()).unwrap();
                                res_str
                            },
                            Payload::Borrowed(contents) => {
                                // println!("Borrowed: {contents:?}");
                                let res_str = String::from_utf8(contents.to_vec()).unwrap();
                                res_str
                            },
                            Payload::Bytes(contents) => {
                                // println!("Bytes: {contents:?}");
                                let res_str = String::from_utf8(contents.to_vec()).unwrap();
                                res_str
                            },
                            Payload::Owned(contents) => {
                                // println!("Owned: {contents:?}");
                                let res_str = String::from_utf8(contents.to_vec()).unwrap();
                                res_str
                            }
                        };
                        // println!("Message: {}", message);

                        match serde_json::from_str::<MoonResponse>(message.as_str()) {
                            Ok(response) => {
                                match response.clone() {
                                    MoonResponse::MoonResult { result, .. } => {
                                        println!("Received Result: {:?}", serde_json::to_string(&result));
                                    },
                                    MoonResponse::Notification { method, params, .. } => {
                                        match method {
                                            NotificationMethod::NotifyProcStatUpdate {..} => {}
                                            _ => {
                                                println!("MoonNotification: {{\n   {:?},\n    {:?}\n}}", serde_json::to_string(&method), serde_json::to_string(&params));
                                            }
                                        }
                                    },
                                    MoonResponse::MoonError { error, .. } => {
                                        println!("MoonError: {:?}", serde_json::to_string(&error).unwrap());
                                    },
                                }
                                match ws_reader_sender.send(response).await {
                                    Ok(()) => continue,
                                    Err(e) => println!("Unable to send to ws_reader_sender: {}", e.to_string()),
                                }
                            },
                            Err(e) => {
                                println!("----------------------------MESSAGE NOT PARSED----------------------------");
                                println!("Message Length: {}", message.len());
                                println!("{}", message);
                                println!("--------------------------------------------------------------------------");
                            }
                        }

                    },
                    Err(e) => eprintln!("Error while reading frame: {}", e.to_string()),
                }
            }
            // while let Some(message) = moon_socket_stream.next().await {
            //     // Check if we've received a shutdown signal and leave the while loop if so
            //     match shutdown_receiver.try_recv() {
            //         Ok(should_shutdown) => {
            //             if should_shutdown {
                            // match websocket_shutdown_sender.send(true).await {
                            //     Ok(_) => {
                            //         break;
                            //     },
                            //     Err(e) => {
                            //         println!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
                            //     },
                            // }
                            // break;
            //             }
            //         },
            //         Err(_) => {},
            //     }
            //     match message {
            //         Ok(msg) => {
            //             if msg.len() == 0 {
            //                 continue;
            //             }
            //             let message = msg.into_text().unwrap();
            //             if debug {
            //                 println!("Received: {}", message);
            //             }
            //             let parsed = serde_json::from_str(&message);
            //             match parsed {
            //                 Ok(message) => match ws_reader_sender.send(message).await {
            //                     Ok(()) => continue,
            //                     Err(_) => println!("Unable to send to ws_reader_sender"),
            //                 },
                            // Err(_) => {
                            //     println!("----------------------------MESSAGE NOT PARSED----------------------------");
                            //     println!("Message Length: {}", message.len());
                            //     println!("{}", message);
                            //     println!("--------------------------------------------------------------------------");
                            // },
            //             }
            //         }
            //         Err(e) => {
            //             eprintln!("Error message from moonraker socket: {}", e.to_string());
            //         },
            //     }
            // }
        });

        Self {
            write_stream: ws_writer_sender,
            read_stream: ws_reader_receiver,
            shutdown_sender,
        }
    }
    /// Creates a new `FastMoonConn` instance and establishes a WebSocket connection to the specified `url`.
    ///
    /// # Arguments
    ///
    /// * `url` - A `String` containing the URL of the Moonraker instance to connect to.
    /// * `writer_buffer_size` - The size of the buffer used to store outgoing messages.
    /// * `reader_buffer_size` - The size of the buffer used to store incoming messages.
    ///
    /// # Returns
    ///
    /// A new `FastMoonConn` instance.
    // pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Self {
    pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Self {
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
                        if msg.len() == 0 {
                            continue;
                        }
                        let message = msg.into_text().unwrap();
                        if debug {
                            println!("Received: {}", message);
                        }
                        let parsed = serde_json::from_str(&message);
                        match parsed {
                            Ok(message) => match ws_reader_sender.send(message).await {
                                Ok(()) => continue,
                                Err(_) => println!("Unable to send to ws_reader_sender"),
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

        Self {
            write_stream: ws_writer_sender,
            read_stream: ws_reader_receiver,
            shutdown_sender,
        }
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.shutdown_sender.send(true).await {
            Ok(_) => {
                Ok(())
            },
            Err(e) => {
                Err(format!("Failed to send shutdown signal to the moonraker message stream: {}", e.to_string()).into())
            },
        }
    }
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
    pub async fn send(&mut self, message: MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
        // self.write_stream.send(message).await.map_err(|e| e.into())?;
        match self.write_stream.send(message).await {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Error sending `{:?}` to the `FastMoonConn` request channel: {}", e.0, e.to_string()).into())
        }
        // Ok(())
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
    pub async fn send_reserved(&mut self, message: MoonRequest) -> Result<(), MoonSendError<()>> {
        let permit = self.reserve().await.map_err(|e| e.into())?;
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
    pub async fn recv(&mut self) -> Option<MoonResponse> {
        self.read_stream.recv().await
    }
    /// Sends message and then waits for the printer to send an Ok message back
    pub async fn send_wait_for_ok(&mut self, message: MoonRequest) -> Result<(), Box<dyn std::error::Error>> {
        let this_id = message.id;
        self.send(message).await?;
        loop {
            match self.recv().await {
                Some(msg) => {
                    match msg {
                        // MoonMSG::MoonResult { id, result, .. } => {
                        MoonResponse::MoonResult { id, result, .. } => {
                            if id == this_id {
                                match result {
                                    MoonResultData::Ok(..) => {
                                        return Ok(());
                                    },
                                    _ => continue,
                                }
                            }
                        },
                        _ => continue,
                    }
                },
                None => continue,
            }
        }
    }
    pub async fn send_listen(&mut self, message: MoonRequest) -> Result<MoonResponse, Box<dyn std::error::Error>> {
        let this_id = message.id;
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
                            match id {
                                Some(id) => {
                                    if id == this_id {
                                        return Ok(MoonResponse::MoonError { jsonrpc: crate::JsonRpcVersion::V2, error, id: Some(id) });
                                    }
                                },
                                None => continue,
                            }
                        },
                        MoonResponse::Notification { .. } => {},
                    }
                },
                None => continue,
            }
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
                    _ => Err("Error in `FastMoonConn::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.".into())
                }
            },
            _ => Err("Error in `FastMoonConn::get_printer_info`: did not receive a MoonMSG::MoonResult response, but should have. This is a bug.".into()),
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