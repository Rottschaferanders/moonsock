// use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};
// // use serde::{Serialize, de::DeserializeOwned, Deserialize, Deserializer, Serializer};
// use serde::{Serialize, de::DeserializeOwned};
// use url::Url;
// use std::collections::HashMap;
// use tokio::{
//     time::Duration,
//     sync::{
//         mpsc::{
//             self,
//             error::SendError,
//         },
//         oneshot,
//         Mutex,
//     },
// };
// use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
// use futures_util::{
//     SinkExt, StreamExt
// };

// pub const DEFAULT_WRITER_BUFFER_SIZE: usize = 1000;
// pub const DEFAULT_READER_BUFFER_SIZE: usize = 1000;

// const DEFAULT_SEND_LISTEN_TIMEOUT: Duration = Duration::from_secs(60);


// /// A trait that represents a JSON-RPC message. It provides methods to get and set the message's ID.
// ///
// /// This trait is used by the `JsonRpcWsClient` to track and match requests and responses.
// pub trait JsonRpcMessage {
//     /// Returns the ID of the message.
//     fn id(&self) -> Option<u32>;

//     /// Sets the ID of the message.
//     fn set_id(&mut self, id: u32);
// }

// // struct JsonRpcMessageWrapper<T> {
// //     pub id: Option<u32>,
// //     pub message: T,
// // }

// // impl<T> Serialize for JsonRpcMessageWrapper<T>
// // where
// //     T: Serialize,
// // {
// //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// //     where
// //         S: Serializer,
// //     {
// //         self.message.serialize(serializer)
// //     }
// // }

// // impl<'de, T> Deserialize<'de> for JsonRpcMessageWrapper<T>
// // where
// //     T: Deserialize<'de>,
// // {
// //     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// //     where
// //         D: Deserializer<'de>,
// //     {
// //         let value = serde_json::Value::deserialize(deserializer)?;
// //         let id = value.get("id").and_then(|id| id.as_u64().map(|id| id as u32));
// //         let message = T::deserialize(value).map_err(|_| serde::de::Error::custom("Failed to deserialize message"))?;
// //         Ok(JsonRpcMessageWrapper { id, message })
// //     }
// // }

// // impl<T> JsonRpcMessage for JsonRpcMessageWrapper<T>
// // where
// //     T: JsonRpcMessage,
// // {
// //     fn id(&self) -> Option<u32> {
// //         self.id
// //     }

// //     fn set_id(&mut self, id: u32) {
// //         self.id = Some(id);
// //         self.message.set_id(id);
// //     }
// // }



// pub struct JsonRpcWsClient<REQ, RES>
// where
//     REQ: Serialize + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
//     RES: DeserializeOwned + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
// {
//     /// The sender for sending messages to the WebSocket server.
//     ws_writer: mpsc::Sender<REQ>,
//     /// The receiver for receiving messages from the WebSocket server.
//     ws_reader: mpsc::Receiver<RES>,
//     /// The sender for sending shutdown signals to the WebSocket server.
//     shutdown_sender: mpsc::Sender<()>,
//     /// A counter for generating unique IDs for messages.
//     id_counter: AtomicUsize,
//     /// A map of pending requests, where the key is the message ID and the value is a sender for sending the response.
//     pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<RES>>>>,
// }

// /// Implements the `Drop` trait for `JsonRpcWsClient`. This allows the client to be dropped and its resources to be released.
// impl<REQ, RES> Drop for JsonRpcWsClient<REQ, RES>
// where
//     REQ: Serialize + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
//     RES: DeserializeOwned + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
// {
//     fn drop(&mut self) {
//         let shutdown_sender = self.shutdown_sender.clone();
//         tokio::spawn(async move {
//             shutdown_sender.send(()).await.ok();
//         });
//     }
// }


// // Or maybe we rethink the below code which is our underlying JSON-RPC library 
// // that is the reason we need the trait in the first place. We could create a 
// // `JsonRpcNotification` type and a new method on `JsonRpcWsClient` called 
// // `recv_notification` that returns the `JsonRpcNotification` type. We could 
// // check if a message we receive from the websocket can be parsed by the users 
// // structs if so, send it to the usual channel. If not, try to parse it into 
// // `JsonRpcNotification` and if it succeeds, send it along a new channel that 
// // is only for notifications. Then the `recv_notification` method can be used 
// // to read notifications from the new channel.
// impl<REQ, RES> JsonRpcWsClient<REQ, RES>
// where
//     REQ: Serialize + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
//     RES: DeserializeOwned + JsonRpcMessage + std::fmt::Debug + std::marker::Send + 'static,
// {
//     /// Creates a new `JsonRpcWsClient` instance.
//     ///
//     /// This method establishes a WebSocket connection to the specified URL and sets up the necessary channels for sending and receiving messages.
//     ///
//     /// The `JsonRpcWsClient` uses the `tracing` crate for debug and error messages.
//     ///
//     /// # Parameters
//     ///
//     /// * `url`: The URL of the WebSocket server to connect to.
//     /// * `writer_buffer_size`: The size of the buffer for sending messages. If `None`, the default buffer size is used.
//     /// * `reader_buffer_size`: The size of the buffer for receiving messages. If `None`, the default buffer size is used.
//     pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>) -> Result<Self, Box<dyn std::error::Error>> {
//         let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
//         let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);
    
//         let (ws_writer_sender, mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
//         let (shutdown_sender, mut shutdown_receiver) = tokio::sync::mpsc::channel(10);
    
//         let connect_addr = Url::parse(&url)?;
    
//         let (ws_stream, _) = connect_async(&connect_addr).await?;
        
//         tracing::debug!("WebSocket handshake has been successfully completed");
    
//         let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
//         let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();
    
//         tokio::spawn(async move {
//             loop {
//                 tokio::select! {
//                     Some(msg) = ws_writer_receiver.recv() => {
//                         match serde_json::to_vec(&msg) {
//                             Ok(mut vec) => {
//                                 vec.truncate(vec.len());
    
//                                 match Pin::new(&mut moon_socket_sink).send(Message::binary(vec)).await {
//                                     Ok(_) => {},
//                                     Err(e) => eprintln!("Error sending to moon_socket_sink: {}", e),
//                                 }
//                             },
//                             Err(e) => eprintln!("Error serializing request: {}", e),
//                         }
//                     },
//                     _ = websocket_shutdown_receiver.recv() => {
//                         break;
//                     },
//                 }
//             }
//         });
    
//         let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
        
//         let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<RES>>::new()));
//         let pending_requests_clone = Arc::clone(&pending_requests);

//         tokio::spawn(async move {
//             loop {
//                 tokio::select! {
//                     Some(message) = moon_socket_stream.next() => {
//                         match message {
//                             Ok(msg) => {
//                                 if msg.len() == 0 {
//                                     continue;
//                                 }
//                                 let message_txt = match msg.into_text() {
//                                     Ok(txt) => txt,
//                                     Err(e) => {
//                                         tracing::error!("Error converting message to text: {}", e);
//                                         continue;
//                                     }
//                                 };
//                                 tracing::debug!("Received: {}", message_txt);
//                                 // let parsed = serde_json::from_str::<JsonRpcMessageWrapper<RES>>(&message_txt);
//                                 let parsed = serde_json::from_str::<RES>(&message_txt);
//                                 match parsed {
//                                     Ok(wrapped_message) => {
//                                         match wrapped_message.id() {
//                                             Some(id) => {
//                                                 if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                                     // tx.send(wrapped_message.message).ok();
//                                                     tx.send(wrapped_message).ok();
//                                                 } else {
//                                                     // ws_reader_sender.send(wrapped_message.message).await.ok();
//                                                     ws_reader_sender.send(wrapped_message).await.ok();
//                                                 }
//                                             },
//                                             None => {
//                                                 // ws_reader_sender.send(wrapped_message.message).await.ok();
//                                                 ws_reader_sender.send(wrapped_message).await.ok();
//                                             },
//                                         }
//                                     },
//                                     Err(_) => {
//                                         tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
//                                         tracing::error!("Message Length: {}", message_txt.len());
//                                         tracing::error!("{}", message_txt);
//                                         tracing::error!("--------------------------------------------------------------------------");
//                                     },
//                                 }
//                             }
//                             Err(e) => tracing::error!("Error message from moonraker socket: {}", e.to_string()),
//                         }
//                     },
//                     _ = shutdown_receiver.recv() => {
//                         match websocket_shutdown_sender.send(()).await {
//                             Ok(_) => {},
//                             Err(e) => {
//                                 tracing::error!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
//                             },
//                         }
//                         break;
//                     },
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

//     /// Receives a message from the WebSocket server.
//     pub async fn recv(&mut self) -> Option<RES> {
//         match self.ws_reader.recv().await {
//             Some(message) => Some(message),
//             None => None,
//         }
//     }
    

//     /// Sends a message to the WebSocket server.
//     pub async fn send(&mut self, message: REQ) -> Result<(), SendError<REQ>> {
//         self.ws_writer.send(message).await?;
//         Ok(())
//     }
    

//     /// Sends a message to the WebSocket server and waits for a response.
//     pub async fn send_listen(&mut self, mut message: REQ) -> Result<RES, Box<dyn std::error::Error>> {
//         let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
//         message.set_id(id);

//         let (tx, rx) = oneshot::channel();
//         self.pending_requests.lock().await.insert(id, tx);

//         self.send(message).await?; // Send the message

//         match tokio::time::timeout(DEFAULT_SEND_LISTEN_TIMEOUT, rx).await { // Example timeout of 5 seconds
//             Ok(Ok(response)) => Ok(response),
//             Ok(Err(_)) => Err("Channel closed unexpectedly".to_string().into()),
//             Err(_) => Err("Timeout waiting for response".to_string().into()),
//         }
//     }
// }