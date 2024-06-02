use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};
// use futures_util::{SinkExt, StreamExt, pin_mut};
use serde::{Serialize, de::DeserializeOwned, Deserialize, Deserializer, Serializer, ser::SerializeMap};
// use tokio::sync::{mpsc, oneshot};
use tokio::time::Duration;
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};
use url::Url;


// use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::Arc;
// use axum::http::response;
// use spinoff::spinners::Moon;
// use tokio::sync::Mutex;
// use tokio::time::sleep;
use std::collections::HashMap;
// use std::error::Error as StdError;

// use std::time::{Duration, Instant};
use tokio::{
    // time::sleep,
    // io::{
    //     AsyncWriteExt, 
    //     stdin, stdout,
    // },
    sync::{
        mpsc::{
            self,
            error::SendError,
            // Permit,
        },
        oneshot,
        Mutex,
    },
};
// use tokio_util::codec::{FramedRead, LinesCodec};
// use spinoff::{Spinner, spinners, Color};


use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use core::pin::Pin;
use futures_util::{
    // sink::*, 
    SinkExt, StreamExt
};
// use url::Url;

// use crate::response::{PrinterState, ServerInfo};
// use crate::{
//     response::MoonResultData, 
//     MoonErrorContent, MoonMethod, MoonParam, MoonRequest, MoonResponse, 
//     PrinterInfoResponse, 
//     PrinterObject,
// };

// use crate::MoonResponse;

pub const DEFAULT_WRITER_BUFFER_SIZE: usize = 1000;
pub const DEFAULT_READER_BUFFER_SIZE: usize = 1000;

const DEFAULT_SEND_LISTEN_TIMEOUT: Duration = Duration::from_secs(60);

// ... existing imports

// JsonRpcWsClient for generic websocket communication
// pub struct JsonRpcWsClient<O, I>
// where
//     O: Serialize,
//     I: DeserializeOwned,
// {
//     ws_writer: mpsc::Sender<O>,
//     ws_reader: mpsc::Receiver<I>,
//     shutdown_sender: mpsc::Sender<()>,
//     // id_counter: AtomicUsize,
//     // pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<I>>>>,
// }

// impl<O, I> JsonRpcWsClient<O, I>
// where
//     O: Serialize + std::fmt::Debug + std::marker::Send + 'static,
//     I: DeserializeOwned + std::fmt::Debug + std::marker::Send + 'static,
// {
//     pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Result<Self, Box<dyn std::error::Error>> {
//         let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
//         let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);
//         let (ws_writer, mut ws_writer_receiver) = mpsc::channel(writer_buffer_size);
//         let (shutdown_sender, mut shutdown_receiver) = mpsc::channel(10);

//         let connect_addr = Url::parse(&url)?;
//         let (ws_stream, _) = connect_async(&connect_addr)
//             .await
//             .map_err(|e| format!("Error connecting to websocket: {}", e))?;

//         if debug {
//             println!("WebSocket handshake has been successfully completed");
//         }

//         let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
//         let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

//         tokio::spawn(async move {
//             while let Some(msg) = ws_writer_receiver.recv().await {
//                 match websocket_shutdown_receiver.try_recv() {
//                     Ok(()) => break,
//                     Err(_) => {}
//                 }

//                 match serde_json::to_vec(&msg) {
//                     Ok(mut vec) => {
//                         vec.truncate(vec.len());
//                         match Pin::new(&mut moon_socket_sink).send(Message::binary(vec)).await {
//                             Ok(_) => {},
//                             Err(e) => eprintln!("Error sending to moon_socket_sink: {}", e),
//                         }
//                     },
//                     Err(e) => eprintln!("Error serializing request: {}", e),
//                 }
//             }
//         });

//         let (ws_reader_sender, ws_reader_receiver) = mpsc::channel(reader_buffer_size);
//         tokio::spawn(async move {
//             while let Some(message) = moon_socket_stream.next().await {
//                 match shutdown_receiver.try_recv() {
//                     Ok(()) => {
//                         match websocket_shutdown_sender.send(()).await {
//                             Ok(_) => {},
//                             Err(e) => {
//                                 println!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
//                             },
//                         }
//                         break;
//                     },
//                     Err(_) => {},
//                 }

//                 match message {
//                     Ok(msg) => {
//                         if msg.len() == 0 {
//                             continue;
//                         }
//                         let message = msg.into_text().unwrap();
//                         if debug {
//                             println!("Received: {}", message);
//                         }
//                         match serde_json::from_str::<I>(&message) {
//                             Ok(message) => {
//                                 ws_reader_sender.send(message).await.ok();
//                             }
//                             Err(e) => {
//                                 eprintln!("Error deserializing response: {}", e);
//                                 // Here, you can choose whether to continue or break based on your error handling strategy.
//                             }
//                         }
//                     },
//                     Err(e) => {
//                         eprintln!("Error message from moonraker socket: {}", e.to_string());
//                     },
//                 }
//             }
//         });

//         Ok(JsonRpcWsClient {
//             ws_writer,
//             ws_reader: ws_reader_receiver,
//             shutdown_sender,
//         })
//     }

//     // pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Result<Self, Box<dyn std::error::Error>> {
//     //     let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
//     //     let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);

//     //     let (ws_writer_sender, mut ws_writer_receiver) = mpsc::channel(writer_buffer_size);
//     //     let (shutdown_sender, mut shutdown_receiver) = mpsc::channel(10);

//     //     let connect_addr = Url::parse(&url).unwrap();

//     //     let (ws_stream, _) = connect_async(&connect_addr)
//     //         .await
//     //         .map_err(|e| format!("Error connecting to websocket: {}", e))?;

//     //     if debug {
//     //         println!("WebSocket handshake has been successfully completed");
//     //     }

//     //     let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
//     //     let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

//     //     // Spawns Moonraker Websocket writer thread
//     //     tokio::spawn(async move {
//     //         while let Some(msg) = ws_writer_receiver.recv().await {
//     //             match websocket_shutdown_receiver.try_recv() {
//     //                 Ok(()) => {
//     //                     break;
//     //                 },
//     //                 Err(_) => {},
//     //             }

//     //             match serde_json::to_vec(&msg) {
//     //                 Ok(mut vec) => {
//     //                     vec.truncate(vec.len());

//     //                     match Pin::new(&mut moon_socket_sink).send(Message::binary(vec)).await {
//     //                         Ok(_) => {},
//     //                         Err(e) => eprintln!("Error sending to moon_socket_sink: {}", e),
//     //                     }
//     //                 },
//     //                 Err(e) => eprintln!("Error serializing request: {}", e),
//     //             }
//     //         }
//     //     });

//     //     let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
        
//     //     let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<I>>::new()));
//     //     let pending_requests_clone = Arc::clone(&pending_requests);
//     //     // Spawns Moonraker Websocket reader thread
//     //     tokio::spawn(async move {
//     //         while let Some(message) = moon_socket_stream.next().await {
//     //             match shutdown_receiver.try_recv() {
//     //                 Ok(()) => {
//     //                     match websocket_shutdown_sender.send(()).await {
//     //                         Ok(_) => {},
//     //                         Err(e) => {
//     //                             println!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
//     //                         },
//     //                     }
//     //                     break;
//     //                 },
//     //                 Err(_) => {},
//     //             }
//     //             match message {
//     //                 Ok(msg) => {
//     //                     if msg.len() == 0 {
//     //                         continue;
//     //                     }
//     //                     let message = msg.into_text().unwrap();
//     //                     if debug {
//     //                         println!("Received: {}", message);
//     //                     }
//     //                     let parsed = serde_json::from_str(&message);
//     //                     match parsed {
//     //                         Ok(message) => {
//     //                             ws_reader_sender.send(message).await.ok();
//     //                             // match message {
//     //                             //     MoonResponse::MoonResult { id, .. } | MoonResponse::MoonError { id: Some(id), .. } => {
//     //                             //         if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//     //                             //             tx.send(message).ok();
//     //                             //         } else {
//     //                             //             ws_reader_sender.send(message).await.ok();
//     //                             //         }
//     //                             //     },
//     //                             //     _ => {},
//     //                             // }
//     //                         },
//     //                         Err(_) => {
//     //                             println!("----------------------------MESSAGE NOT PARSED----------------------------");
//     //                             println!("Message Length: {}", message.len());
//     //                             println!("{}", message);
//     //                             println!("--------------------------------------------------------------------------");
//     //                         },
//     //                     }
//     //                 }
//     //                 Err(e) => eprintln!("Error message from moonraker socket: {}", e.to_string()),
//     //             }
//     //         }
//     //     });

//     //     Ok(Self {
//     //         // write_stream: ws_writer_sender,
//     //         // read_stream: ws_reader_receiver,
//     //         ws_writer: ws_writer_sender,
//     //         ws_reader: ws_reader_receiver,
//     //         shutdown_sender,
//     //         id_counter: AtomicUsize::new(1),
//     //         pending_requests,
//     //     })
//     // }

//     pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         match self.shutdown_sender.send(()).await {
//             Ok(()) => Ok(()),
//             Err(e) => Err(format!("Failed to send shutdown signal to the moonraker message stream: {}", e.to_string()).into()),
//         }
//     }

//     pub async fn recv(&mut self) -> Option<I> {
//         self.ws_reader.recv().await
//     }

//     pub async fn send(&mut self, message: O) -> Result<(), SendError<O>> {
//         self.ws_writer.send(message).await?;
//         Ok(())
//     }

//     // pub async fn send_listen(&mut self, mut message: MoonRequest) -> Result<MoonResponse, MoonSendError<MoonRequest>> {
//     // pub async fn send_listen(&mut self, mut message: O) -> Result<I, Box<dyn std::error::Error>> {
//     //     let this_id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
//     //     message.id = this_id;

//     //     let (tx, rx) = oneshot::channel();
//     //     self.pending_requests.lock().await.insert(this_id, tx);

//     //     self.send(message).await?; // Send the message

//     //     // Await the response on the dedicated channel
//     //     match rx.await {
//     //         // Ok(response) => match response {
//     //         //     MoonResponse::MoonResult {..} 
//     //         //     | MoonResponse::Notification {..} => Ok(response),
//     //         //     MoonResponse::MoonError {error, ..} => Err(MoonSendError::MoonError(error)),
//     //         // },
//     //         Ok(r) => Ok(r),
//     //         // Err(e) => Err(MoonSendError::String(format!("Oneshot channel error: {}", e.to_string()))),
//     //         Err(e) => Err(format!("Oneshot channel error: {}", e.to_string()).into()),
//     //     }
//     // }
    
//     // pub async fn send_wait_for_ok(&mut self, message: MoonRequest) -> Result<(), MoonSendError<MoonRequest>> {
//     //     let res = self.send_listen(message).await?;
//     //     match res {
//     //         MoonResponse::MoonResult { result: MoonResultData::Ok(..), .. } => Ok(()),
//     //         // Should never be possible to get the `MoonResponse::MoonError` variant as long as the logic of `send_listen` never changes, but
//     //         // for correctness reasons we should still check for it.
//     //         MoonResponse::MoonError { error, .. } => Err(MoonSendError::MoonError(error)),
//     //         _ => Err(MoonSendError::String(format!("Expected an Ok response got: {res:?}"))),
//     //     }
//     // }
// }













// pub struct JsonRpcWsClient<O, I>
// where
//     O: Serialize + std::fmt::Debug + std::marker::Send + 'static,
//     I: DeserializeOwned + std::fmt::Debug + std::marker::Send + 'static,
// {
//     ws_writer: mpsc::Sender<O>,
//     ws_reader: mpsc::Receiver<I>,
//     shutdown_sender: mpsc::Sender<()>,
//     id_counter: AtomicUsize,
//     pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<I>>>>,
// }

// impl<O, I> JsonRpcWsClient<O, I>
// where
//     O: Serialize + std::fmt::Debug + std::marker::Send + 'static,
//     I: DeserializeOwned + std::fmt::Debug + std::marker::Send + 'static,
// {
//     pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Result<Self, Box<dyn std::error::Error>> {
//         let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
//         let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);

//         let (ws_writer_sender, mut ws_writer_receiver) = mpsc::channel(writer_buffer_size);
//         let (shutdown_sender, mut shutdown_receiver) = mpsc::channel(10);

//         let connect_addr = Url::parse(&url).unwrap();

//         let (ws_stream, _) = connect_async(&connect_addr)
//             .await
//             .map_err(|e| format!("Error connecting to websocket: {}", e))?;

//         if debug {
//             println!("WebSocket handshake has been successfully completed");
//         }

//         let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
//         let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

//         // Spawns Moonraker Websocket writer thread
//         tokio::spawn(async move {
//             while let Some(msg) = ws_writer_receiver.recv().await {
//                 match websocket_shutdown_receiver.try_recv() {
//                     Ok(()) => {
//                         break;
//                     },
//                     Err(_) => {},
//                 }

//                 match serde_json::to_vec(&msg) {
//                     Ok(mut vec) => {
//                         vec.truncate(vec.len());

//                         match Pin::new(&mut moon_socket_sink).send(Message::binary(vec)).await {
//                             Ok(_) => {},
//                             Err(e) => eprintln!("Error sending to moon_socket_sink: {}", e),
//                         }
//                     },
//                     Err(e) => eprintln!("Error serializing request: {}", e),
//                 }
//             }
//         });

//         let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
        
//         let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<I>>::new()));
//         let pending_requests_clone = Arc::clone(&pending_requests);
//         // Spawns Moonraker Websocket reader thread
//         tokio::spawn(async move {
//             while let Some(message) = moon_socket_stream.next().await {
//                 match shutdown_receiver.try_recv() {
//                     Ok(()) => {
//                         match websocket_shutdown_sender.send(()).await {
//                             Ok(_) => {},
//                             Err(e) => {
//                                 println!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
//                             },
//                         }
//                         break;
//                     },
//                     Err(_) => {},
//                 }
//                 match message {
//                     Ok(msg) => {
//                         if msg.len() == 0 {
//                             continue;
//                         }
//                         let message = msg.into_text().unwrap();
//                         if debug {
//                             println!("Received: {}", message);
//                         }
//                         let parsed = serde_json::from_str(&message);
//                         match parsed {
//                             Ok(message) => {
//                                 match message {
//                                     MoonResponse::MoonResult { id, .. } | MoonResponse::MoonError { id: Some(id), .. } => {
//                                         if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                             tx.send(message).ok();
//                                         } else {
//                                             ws_reader_sender.send(message).await.ok();
//                                         }
//                                     },
//                                     _ => {},
//                                 }
//                             },
//                             Err(_) => {
//                                 println!("----------------------------MESSAGE NOT PARSED----------------------------");
//                                 println!("Message Length: {}", message.len());
//                                 println!("{}", message);
//                                 println!("--------------------------------------------------------------------------");
//                             },
//                         }
//                     }
//                     Err(e) => eprintln!("Error message from moonraker socket: {}", e.to_string()),
//                 }
//             }
//         });

//         Ok(JsonRpcWsClient {
//             ws_writer: ws_writer_sender,
//             ws_reader: ws_reader_receiver,
//             shutdown_sender,
//             id_counter: AtomicUsize::new(1),
//             pending_requests,
//         })
//     }

//     pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         match self.shutdown_sender.send(()).await {
//             Ok(()) => Ok(()),
//             Err(e) => Err(format!("Failed to send shutdown signal to the moonraker message stream: {}", e.to_string()).into()),
//         }
//     }

//     pub async fn recv(&mut self) -> Option<I> {
//         self.ws_reader.recv().await
//     }

//     pub async fn send(&mut self, message: O) -> Result<(), mpsc::error::SendError<O>> {
//         self.ws_writer.send(message).await
//     }

//     pub async fn send_listen(&mut self, mut message: O, id: u32) -> Result<I, Box<dyn std::error::Error>> {
//         let (tx, rx) = oneshot::channel();
//         self.pending_requests.lock().await.insert(id, tx);

//         self.send(message).await?; // Send the message

//         match tokio::time::timeout(DEFAULT_SEND_LISTEN_TIMEOUT, rx).await { // Example timeout of 5 seconds
//             Ok(Ok(response)) => Ok(response),
//             Ok(Err(_)) => Err("Channel closed unexpectedly".into()),
//             Err(_) => Err("Timeout waiting for response".into()),
//         }
//     }

//     pub async fn send_wait_for_ok(&mut self, message: O, id: u32) -> Result<(), MoonSendError<O>> {
//         let res = self.send_listen(message, id).await?;
//         match res {
//             MoonResponse::MoonResult { result: MoonResultData::Ok(..), .. } => Ok(()),
//             // Should never be possible to get the `MoonResponse::MoonError` variant as long as the logic of `send_listen` never changes, but
//             // for correctness reasons we should still check for it.
//             MoonResponse::MoonError { error, .. } => Err(MoonSendError::MoonError(error)),
//             _ => Err(MoonSendError::String(format!("Expected an Ok response got: {res:?}"))),
//         }
//     }
// }








/// A trait that represents a JSON-RPC message. It provides methods to get and set the message's ID.
pub trait JsonRpcMessage {
    /// Returns the ID of the message.
    fn id(&self) -> Option<u32>;

    /// Sets the ID of the message.
    fn set_id(&mut self, id: u32);
}

struct JsonRpcMessageWrapper<T> {
    pub id: Option<u32>,
    pub message: T,
}

impl<T> Serialize for JsonRpcMessageWrapper<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.message.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for JsonRpcMessageWrapper<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let id = value.get("id").and_then(|id| id.as_u64().map(|id| id as u32));
        let message = T::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to deserialize message"))?;
        Ok(JsonRpcMessageWrapper { id, message })
    }
}

impl<T> JsonRpcMessage for JsonRpcMessageWrapper<T>
where
    T: JsonRpcMessage,
{
    fn id(&self) -> Option<u32> {
        self.id
    }

    fn set_id(&mut self, id: u32) {
        self.id = Some(id);
        self.message.set_id(id);
    }
}


/// A WebSocket client for sending and receiving JSON-RPC messages.
///
/// **Important:** When using the `JsonRpcWsClient`, it is crucial that your top-level serde types (i.e., the types you use for `REQ` and `RES`) implement the `JsonRpcMessage` trait correctly. This trait provides methods for getting and setting the message's ID, which is essential for tracking and matching requests and responses.
///
/// To implement the `JsonRpcMessage` trait, you need to provide implementations for the `id` and `set_id` methods. The `id` method should return the current ID of the message, and the `set_id` method should set the ID of the message.
///
/// Here's an example of how you might implement the `JsonRpcMessage` trait for a simple serde type:
/// ```rust
/// #[derive(Serialize, Deserialize, Debug)]
/// struct MyRequest {
///     jsonrpc: String,
///     method: String,
///     params: Vec<String>,
///     id: Option<u32>,
/// }
///
/// impl JsonRpcMessage for MyRequest {
///     fn id(&self) -> Option<u32> {
///         self.id
///     }
///
///     fn set_id(&mut self, id: u32) {
///         self.id = Some(id);
///     }
/// }
/// ```
/// If you do not implement the `JsonRpcMessage` trait correctly for your serde types, the program will not compile. This is because the `JsonRpcWsClient` relies on the trait's methods to function correctly.
///
/// By implementing the `JsonRpcMessage` trait correctly, you ensure that your serde types can be used with the `JsonRpcWsClient` to send and receive JSON-RPC messages.
pub struct JsonRpcWsClient<REQ, RES>
where
    REQ: Serialize + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
    RES: DeserializeOwned + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
{
    /// The sender for sending messages to the WebSocket server.
    ws_writer: mpsc::Sender<REQ>,
    /// The receiver for receiving messages from the WebSocket server.
    ws_reader: mpsc::Receiver<RES>,
    /// The sender for sending shutdown signals to the WebSocket server.
    shutdown_sender: mpsc::Sender<()>,
    /// A counter for generating unique IDs for messages.
    id_counter: AtomicUsize,
    /// A map of pending requests, where the key is the message ID and the value is a sender for sending the response.
    pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<RES>>>>,
}

/// Implements the `Drop` trait for `JsonRpcWsClient`. This allows the client to be dropped and its resources to be released.
impl<REQ, RES> Drop for JsonRpcWsClient<REQ, RES>
where
    REQ: Serialize + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
    RES: DeserializeOwned + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
{
    fn drop(&mut self) {
        let shutdown_sender = self.shutdown_sender.clone();
        tokio::spawn(async move {
            shutdown_sender.send(()).await.ok();
        });
    }
}

/// Implements methods for `JsonRpcWsClient`.
impl<REQ, RES> JsonRpcWsClient<REQ, RES>
where
    REQ: Serialize + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
    RES: DeserializeOwned + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
{
    /// Creates a new `JsonRpcWsClient` instance.
    pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>, debug: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
        let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);
    
        let (ws_writer_sender, mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
        let (shutdown_sender, mut shutdown_receiver) = tokio::sync::mpsc::channel(10);
    
        let connect_addr = Url::parse(&url)?;
    
        let (ws_stream, _) = connect_async(&connect_addr).await?;
    
        if debug {
            println!("WebSocket handshake has been successfully completed");
        }
    
        let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
        let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();
    
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = ws_writer_receiver.recv() => {
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
                    },
                    _ = websocket_shutdown_receiver.recv() => {
                        break;
                    },
                }
            }
        });
    
        let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
        
        let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<RES>>::new()));
        let pending_requests_clone = Arc::clone(&pending_requests);

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(message) = moon_socket_stream.next() => {
                        match message {
                            Ok(msg) => {
                                if msg.len() == 0 {
                                    continue;
                                }
                                let message_txt = match msg.into_text() {
                                    Ok(txt) => txt,
                                    Err(e) => {
                                        tracing::error!("Error converting message to text: {}", e);
                                        continue;
                                    }
                                };
                                if debug {
                                    tracing::info!("Received: {}", message_txt);
                                }
                                let parsed = serde_json::from_str::<JsonRpcMessageWrapper<RES>>(&message_txt);
                                match parsed {
                                    Ok(wrapped_message) => {
                                        match wrapped_message.id {
                                            Some(id) => {
                                                if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
                                                    tx.send(wrapped_message.message).ok();
                                                } else {
                                                    ws_reader_sender.send(wrapped_message.message).await.ok();
                                                }
                                            },
                                            None => {
                                                ws_reader_sender.send(wrapped_message.message).await.ok();
                                            },
                                        }
                                    },
                                    Err(_) => {
                                        tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
                                        tracing::error!("Message Length: {}", message_txt.len());
                                        tracing::error!("{}", message_txt);
                                        tracing::error!("--------------------------------------------------------------------------");
                                    },
                                }
                            }
                            Err(e) => tracing::error!("Error message from moonraker socket: {}", e.to_string()),
                        }
                    },
                    _ = shutdown_receiver.recv() => {
                        match websocket_shutdown_sender.send(()).await {
                            Ok(_) => {},
                            Err(e) => {
                                tracing::error!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
                            },
                        }
                        break;
                    },
                }
            }
        });
    
        Ok(JsonRpcWsClient {
            ws_writer: ws_writer_sender,
            ws_reader: ws_reader_receiver,
            shutdown_sender,
            id_counter: AtomicUsize::new(1),
            pending_requests,
        })
    }

    /// Shuts down the client and releases its resources.
    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self.shutdown_sender.send(()).await {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("Failed to send shutdown signal to the moonraker message stream: {}", e.to_string()).into()),
        }
    }

    /// Receives a message from the WebSocket server.
    pub async fn recv(&mut self) -> Option<RES> {
        match self.ws_reader.recv().await {
            Some(message) => Some(message),
            None => None,
        }
    }

    /// Sends a message to the WebSocket server.
    pub async fn send(&mut self, message: REQ) -> Result<(), SendError<REQ>> {
        self.ws_writer.send(message).await?;
        Ok(())
    }

    /// Sends a message to the WebSocket server and waits for a response.
    pub async fn send_listen(&mut self, mut message: REQ) -> Result<RES, Box<dyn std::error::Error>> {
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
        message.set_id(id);

        let (tx, rx) = oneshot::channel();
        self.pending_requests.lock().await.insert(id, tx);

        self.send(message).await?; // Send the message

        match tokio::time::timeout(DEFAULT_SEND_LISTEN_TIMEOUT, rx).await { // Example timeout of 5 seconds
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err("Channel closed unexpectedly".to_string().into()),
            Err(_) => Err("Timeout waiting for response".to_string().into()),
        }
    }
}