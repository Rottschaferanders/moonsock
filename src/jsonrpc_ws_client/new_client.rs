use std::sync::{
    atomic::AtomicUsize, atomic::Ordering, 
    // atomic::AtomicBool,
    Arc
};
use serde::{Serialize, Deserialize};
use url::Url;
use std::collections::HashMap;
use tokio::{
    time::Duration,
    time::Instant,
    sync::{
        mpsc::{
            self,
            error::SendError,
        },
        oneshot,
        Mutex,
    },
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{
    SinkExt, StreamExt
};

pub const DEFAULT_WRITER_BUFFER_SIZE: usize = 1000;
pub const DEFAULT_READER_BUFFER_SIZE: usize = 1000;

const DEFAULT_SEND_LISTEN_TIMEOUT: Duration = Duration::from_secs(60);
const DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcRequest {
// pub struct JsonRpcRequest<S: Serialize> {
    /// The JSON-RPC version of the request.
    jsonrpc: JsonRpcVersion,
    /// The method to be called.
    // pub method: String,
    pub method: serde_json::Value,
    // pub method: S,
    /// The parameters to be passed to the method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    // pub params: Option<S>,
    /// The ID of the request.
    id: u32,
}

// impl JsonRpcRequest {
//     pub fn build(method: impl ToString, params: Option<serde_json::Value>) -> Self {
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             method: method.to_string(),
//             params,
//             id: 0,
//         }
//     }
// }

// impl JsonRpcRequest {
//     // pub fn build<M, P>(method: M, params: Option<P>) -> Self
//     pub fn build(method: impl Serialize, params: Option<impl Serialize>) -> Self
//     // where
//     //     // M: Into<String>,
//     //     M: Serialize,
//     //     P: Serialize,
//     {
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             // method: method,
//             method,
//             // params: params.map(|p| serde_json::to_value(p).unwrap()),
//             params,
//             id: 0,
//         }
//     }
// }

// impl JsonRpcRequest {
//     pub fn build<M, P>(method: M, params: P) -> Self
//     where
//         M: Into<String> + Serialize,
//         P: Into<Option<serde_json::Value>>,
//     {
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             method: method.into(),
//             params: params.into(),
//             id: 0,
//         }
//     }
// }

impl JsonRpcRequest {
    pub fn build<M, P>(method: M, params: Option<P>) -> Result<Self, Box<dyn std::error::Error>>
    where
        // M: Into<String>,
        M: Serialize,
        P: Serialize,
    {
        let m = serde_json::to_value(&method).unwrap();
        match m {
            serde_json::Value::String(_) => {
                Ok(Self {
                    jsonrpc: JsonRpcVersion::V2,
                    method: m,
                    params: params.map(|p| serde_json::to_value(p).unwrap()),
                    id: 0,
                })
            },
            _ => Err("Method must be type serializable into a String".into()),
        }
        // Self {
        //     jsonrpc: JsonRpcVersion::V2,
        //     method: method.into(),
        //     params: params.map(|p| serde_json::to_value(p).unwrap()),
        //     id: 0,
        // }
    }
}

#[derive(Debug, Clone)]
pub enum JsonRpcResponse {
    Result(JsonRpcSuccessResponse),
    // ReturnedError(JsonRpcErrorResponse),
    ReturnedError(JsonRpcError),
}

#[derive(thiserror::Error, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.data {
            Some(data) => write!(f, "{}: message: {}: , data: {}", self.code, self.message, data),
            None => write!(f, "{}: {}", self.code, self.message),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcErrorResponse {
    jsonrpc: JsonRpcVersion,
    pub error: JsonRpcError,
    id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcSuccessResponse {
    jsonrpc: JsonRpcVersion,
    pub result: serde_json::Value,
    id: u32,
}


#[derive(Debug, PartialEq)]
enum PendingRequestStatus {
    Clean,
    Dirty {
        timeout: Instant,
        // error_message: String,
        error_message: JsonRpcError,
    },
}


pub struct JsonRpcWsClient {
    /// The sender for sending messages to the WebSocket server.
    ws_writer: mpsc::Sender<JsonRpcRequest>,
    /// The receiver for receiving messages from the WebSocket server.
    ws_reader: mpsc::Receiver<JsonRpcResponse>,
    /// The sender for sending shutdown signals to the WebSocket server.
    shutdown_sender: mpsc::Sender<()>,
    /// A counter for generating unique IDs for messages.
    id_counter: AtomicUsize,
    /// A map of pending requests, where the key is the message ID and the value is a sender for sending the response.
    pending_requests: Arc<Mutex<HashMap<u32, (oneshot::Sender<JsonRpcResponse>, PendingRequestStatus)>>>,
    // dirty_pending_requests: Arc<Mutex<Vec<(u32, Instant)>>>,
    // has_dirty_requests: Arc<AtomicBool>,
    notification_reader: mpsc::Receiver<serde_json::Value>,
}

impl Drop for JsonRpcWsClient {
    fn drop(&mut self) {
        let shutdown_sender = self.shutdown_sender.clone();
        tokio::spawn(async move {
            shutdown_sender.send(()).await.ok();
        });
    }
}

impl JsonRpcWsClient {
    pub async fn new(url: String, writer_buffer_size: Option<usize>, reader_buffer_size: Option<usize>) -> Result<Self, Box<dyn std::error::Error>> {
        let writer_buffer_size = writer_buffer_size.unwrap_or(DEFAULT_WRITER_BUFFER_SIZE);
        let reader_buffer_size = reader_buffer_size.unwrap_or(DEFAULT_READER_BUFFER_SIZE);

        let (ws_writer_sender, mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
        let (notification_sender, notification_reader) = tokio::sync::mpsc::channel(1000);
        let (shutdown_sender, mut shutdown_receiver) = tokio::sync::mpsc::channel(10);

        let connect_addr = Url::parse(&url)?;

        let (ws_stream, _) = connect_async(&connect_addr).await?;

        tracing::debug!("WebSocket handshake has been successfully completed");

        let (websocket_shutdown_sender, mut websocket_shutdown_receiver) = tokio::sync::mpsc::channel(10);
        let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    Some(msg) = ws_writer_receiver.recv() => {
                        tracing::debug!("Sending message: {:?}", serde_json::to_string_pretty(&msg).unwrap());
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

        let pending_requests = Arc::new(Mutex::new(HashMap::<u32, (oneshot::Sender<JsonRpcResponse>, PendingRequestStatus)>::new()));
        let pending_requests_clone = Arc::clone(&pending_requests);

        // let has_dirty_requests = Arc::new(AtomicBool::new(false));
        // let has_dirty_requests_clone = Arc::clone(&has_dirty_requests);

        // let dirty_pending_requests = Arc::new(Mutex::new(Vec::new())); // Renamed from dirty_requests to dirty_pending_requests
        // let dirty_pending_requests_clone = Arc::clone(&dirty_pending_requests);


        tokio::spawn(async move {
            // let mut dirty_pending_requests: HashMap<u32, Instant> = HashMap::new();
            let mut dirty_pending_requests: HashMap<Instant, Vec<u32>> = HashMap::new();
            let mut has_dirty_requests = false;
        
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
                                // tracing::debug!("Received: {}", message_txt);
                                // tracing::trace!("Received: {}", message_txt);
                                let raw_value: serde_json::Value = serde_json::from_str(&message_txt).unwrap();
                                if let Some(_id) = raw_value.get("id") {
                                    // Handle response
                                    let parsed: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
                                    match parsed {
                                        Ok(message) => {
                                            tracing::debug!("Response: {}", message_txt);
                                            let id = message.id;
                                            if let Some((tx, _)) = pending_requests_clone.lock().await.remove(&id) {
                                                tx.send(JsonRpcResponse::Result(message)).ok();
                                            } else {
                                                ws_reader_sender.send(JsonRpcResponse::Result(message)).await.ok();
                                            }
                                        },
                                        Err(_) => {
                                            let parsed: Result<JsonRpcErrorResponse, serde_json::Error> = serde_json::from_str(&message_txt);
                                            match parsed {
                                                Ok(message) => {
                                                    tracing::error!("Error: {}", message_txt);
                                                    match message.id {
                                                        Some(id) => {
                                                            if let Some((tx, _)) = pending_requests_clone.lock().await.remove(&id) {
                                                                tx.send(JsonRpcResponse::ReturnedError(message.error.clone())).ok();
                                                            } else {
                                                                ws_reader_sender.send(JsonRpcResponse::ReturnedError(message.error.clone())).await.ok();
                                                            }
                                                        },
                                                        None => {
                                                            // Handle error with null id
                                                            let mut pending_requests_lock = pending_requests_clone.lock().await;
                                                            if pending_requests_lock.len() == 1 {
                                                                // If there is only one pending request, send the error through the oneshot channel
                                                                let (_, (tx, _)) = pending_requests_lock.drain().next().unwrap();
                                                                tx.send(JsonRpcResponse::ReturnedError(message.error.clone())).ok();
                                                            } else {
                                                                let now = Instant::now();
                                                                for (id, (_, status)) in pending_requests_lock.iter_mut() {
                                                                    if *status == PendingRequestStatus::Clean {
                                                                        *status = PendingRequestStatus::Dirty {
                                                                            timeout: now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT,
                                                                            error_message: message.error.clone(),
                                                                        };
                                                                        dirty_pending_requests.entry(now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT).or_insert(Vec::new()).push(*id);
                                                                    }
                                                                }
                                                                has_dirty_requests = true;
                                                            }
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
                                        },
                                    }
                                } else {
                                    tracing::trace!("Received: {}", message_txt);
                                    // Handle notification
                                    notification_sender.send(raw_value).await.ok();
                                }
                            }
                            Err(e) => tracing::error!("Error message from moonraker socket: {}", e.to_string()),
                        }
                    },

                    // Some(message) = moon_socket_stream.next() => {
                    //     match message {
                    //         Ok(msg) => {
                    //             if msg.len() == 0 {
                    //                 continue;
                    //             }
                    //             let message_txt = match msg.into_text() {
                    //                 Ok(txt) => txt,
                    //                 Err(e) => {
                    //                     tracing::error!("Error converting message to text: {}", e);
                    //                     continue;
                    //                 }
                    //             };
                    //             tracing::debug!("Received: {}", message_txt);
                    //             let raw_value: serde_json::Value = serde_json::from_str(&message_txt).unwrap();
                    //             if let Some(id) = raw_value.get("id") {
                    //                 // Handle response
                    //                 let parsed: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
                    //                 match parsed {
                    //                     Ok(message) => {
                    //                         let id = message.id;
                    //                         if let Some((tx, _)) = pending_requests_clone.lock().await.remove(&id) {
                    //                             tx.send(JsonRpcResponse::Result(message)).ok();
                    //                         } else {
                    //                             ws_reader_sender.send(JsonRpcResponse::Result(message)).await.ok();
                    //                         }
                    //                     },
                    //                     Err(_) => {
                    //                         let parsed: Result<JsonRpcErrorResponse, serde_json::Error> = serde_json::from_str(&message_txt);
                    //                         match parsed {
                    //                             Ok(message) => {
                    //                                 match message.id {
                    //                                     Some(id) => {
                    //                                         if let Some((tx, _)) = pending_requests_clone.lock().await.remove(&id) {
                    //                                             tx.send(JsonRpcResponse::ReturnedError(message.error.clone())).ok();
                    //                                         } else {
                    //                                             ws_reader_sender.send(JsonRpcResponse::ReturnedError(message.error.clone())).await.ok();
                    //                                         }
                    //                                     },
                    //                                     None => {
                    //                                         // Handle error with null id
                    //                                         let mut pending_requests_lock = pending_requests_clone.lock().await;
                    //                                         if pending_requests_lock.len() == 1 {
                    //                                             // If there is only one pending request, send the error through the oneshot channel
                    //                                             let (_, (tx, _)) = pending_requests_lock.drain().next().unwrap();
                    //                                             tx.send(JsonRpcResponse::ReturnedError(message.error.clone())).ok();
                    //                                         } else {
                    //                                             let now = Instant::now();
                    //                                             for (id, (_, status)) in pending_requests_lock.iter_mut() {
                    //                                                 if *status == PendingRequestStatus::Clean {
                    //                                                     *status = PendingRequestStatus::Dirty {
                    //                                                         timeout: now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT,
                    //                                                         error_message: message.error.clone(),
                    //                                                     };
                    //                                                     dirty_pending_requests.insert(*id, now + DEFAULT_DIRTY_PENDING_RESPONSE_TIMEOUT);
                    //                                                 }
                    //                                             }
                    //                                             has_dirty_requests = true;
                    //                                         }
                    //                                     },
                    //                                 }
                    //                             },
                    //                             Err(_) => {
                    //                                 tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
                    //                                 tracing::error!("Message Length: {}", message_txt.len());
                    //                                 tracing::error!("{}", message_txt);
                    //                                 tracing::error!("--------------------------------------------------------------------------");
                    //                             },
                    //                         }
                    //                     },
                    //                 }
                    //             } else {
                    //                 // Handle notification
                    //                 notification_sender.send(raw_value).await.ok();
                    //             }
                    //         }
                    //         Err(e) => tracing::error!("Error message from moonraker socket: {}", e.to_string()),
                    //     }
                    // },
                    _ = shutdown_receiver.recv() => {
                        match websocket_shutdown_sender.send(()).await {
                            Ok(_) => {},
                            Err(e) => {
                                tracing::error!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
                            },
                        }
                        break;
                    },
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        // Check for dirty pending request timeouts
                        if has_dirty_requests {
                            let now = Instant::now();
                            let mut timed_out_requests = Vec::new();
                            for (timeout, ids) in dirty_pending_requests.iter() {
                                if *timeout <= now {
                                    timed_out_requests.push((*timeout, ids.clone()));
                                }
                            }
                            for (timeout, ids) in timed_out_requests {
                                let mut count = ids.len();
                                let mut valid_ids = Vec::new();
                                for id in ids {
                                    if pending_requests_clone.lock().await.contains_key(&id) {
                                        valid_ids.push(id);
                                    } else {
                                        count -= 1;
                                    }
                                }
                                for id in valid_ids {
                                    if let Some((tx, status)) = pending_requests_clone.lock().await.remove(&id) {
                                        if let PendingRequestStatus::Dirty { error_message, .. } = status {
                                            let mut error_message = error_message.clone();
                                            if count > 1 {
                                                error_message.message = format!("Error: This error may not be associated with this request and may have originated from another. One of your requests had an id that the websocket server could not parse. There is probably something wrong with thewebsocket server is parsing ids.\n{}", error_message.message);
                                            }
                                            tx.send(JsonRpcResponse::ReturnedError(error_message)).ok();
                                            // dirty_pending_requests.get_mut(&timeout).unwrap().retain(|val| *val != id);
                                        }
                                    }
                                }
                                dirty_pending_requests.remove(&timeout);
                            }
                            if dirty_pending_requests.is_empty() {
                                has_dirty_requests = false;
                            }
                        }
                    }
                    // _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    //     // Check for dirty pending request timeouts
                    //     if has_dirty_requests {
                    //         let now = Instant::now();
                    //         let mut timed_out_requests = Vec::new();
                    //         for (id, timeout) in dirty_pending_requests.iter() {
                    //             if *timeout <= now {
                    //                 timed_out_requests.push(*id);
                    //             }
                    //         }
                    //         let count = timed_out_requests.len();
                    //         for id in timed_out_requests {
                    //             if let Some((tx, status)) = pending_requests_clone.lock().await.remove(&id) {
                    //                 if let PendingRequestStatus::Dirty { error_message, .. } = status {
                    //                     let mut error_message = error_message.clone();
                    //                     if count > 1 {
                    //                         error_message.message = format!("Error: This error may not be associated with this request and may have originated from another. One of your requests had an id that the websocket server could not parse. There is probably something wrong with the way the websocket server is parsing ids.\n{}", error_message.message);
                    //                     }
                    //                     tx.send(JsonRpcResponse::ReturnedError(error_message)).ok();
                    //                 }
                    //             }
                    //             dirty_pending_requests.remove(&id);
                    //         }
                    //         if dirty_pending_requests.is_empty() {
                    //             has_dirty_requests = false;
                    //         }
                    //     }
                    // }
                }
            }
        });


        // tokio::spawn(async move {
        //     loop {
        //         tokio::select! {
        //             Some(message) = moon_socket_stream.next() => {
        //                 match message {
        //                     Ok(msg) => {
        //                         if msg.len() == 0 {
        //                             continue;
        //                         }
        //                         let message_txt = match msg.into_text() {
        //                             Ok(txt) => txt,
        //                             Err(e) => {
        //                                 tracing::error!("Error converting message to text: {}", e);
        //                                 continue;
        //                             }
        //                         };
        //                         tracing::debug!("Received: {}", message_txt);
        //                         let raw_value: serde_json::Value = serde_json::from_str(&message_txt).unwrap();
        //                         let parsed: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
        //                         match parsed {
        //                             Ok(wrapped_message) => {
        //                                 let id = wrapped_message.id;
        //                                 if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
        //                                     tx.send(JsonRpcResponse::Result(wrapped_message.result)).ok();
        //                                 } else {
        //                                     ws_reader_sender.send(JsonRpcResponse::Result(wrapped_message.result)).await.ok();
        //                                 }
        //                             },
        //                             Err(_) => {
        //                                 let parsed: Result<JsonRpcErrorResponse, serde_json::Error> = serde_json::from_str(&message_txt);
        //                                 match parsed {
        //                                     Ok(wrapped_message) => {
        //                                         let id = wrapped_message.id;
        //                                         if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
        //                                             tx.send(JsonRpcResponse::ReturnedError(serde_json::Value::Null)).ok();
        //                                         } else {
        //                                             ws_reader_sender.send(JsonRpcResponse::ReturnedError(serde_json::Value::Null)).await.ok();
        //                                         }
        //                                     },
        //                                     Err(_) => {
        //                                         tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
        //                                         tracing::error!("Message Length: {}", message_txt.len());
        //                                         tracing::error!("{}", message_txt);
        //                                         tracing::error!("--------------------------------------------------------------------------");
        //                                     },
        //                                 }
        //                             },
        //                         }
        //                     }
        //                     Err(e) => tracing::error!("Error message from moonraker socket: {}", e.to_string()),
        //                 }
        //             },
        //             _ = shutdown_receiver.recv() => {
        //                 match websocket_shutdown_sender.send(()).await {
        //                     Ok(_) => {},
        //                     Err(e) => {
        //                         tracing::error!("Error: Failed to send shutdown signal to websocket task: {}", e.to_string());
        //                     },
        //                 }
        //                 break;
        //             },
        //         }
        //     }
        // });

        Ok(JsonRpcWsClient {
            ws_writer: ws_writer_sender,
            ws_reader: ws_reader_receiver,
            shutdown_sender,
            id_counter: AtomicUsize::new(1),
            pending_requests,
            // dirty_requests: Arc::new(Mutex::new(Vec::<(u32, Instant)>::new())),
            // dirty_pending_requests,
            // has_dirty_requests,
            // has_dirty_requests: Arc::new(AtomicBool::new(false)),
            notification_reader,
        })
    }

    pub async fn recv(&mut self) -> Option<JsonRpcResponse> {
        match self.ws_reader.recv().await {
            Some(message) => Some(message),
            None => None,
        }
    }

    pub async fn listen_for_notification(&mut self) -> Option<serde_json::Value> {
        match self.notification_reader.recv().await {
            Some(message) => Some(message),
            None => None,
        }
    }

    pub async fn send(&mut self, message: JsonRpcRequest) -> Result<(), SendError<JsonRpcRequest>> {
        self.ws_writer.send(message).await?;
        Ok(())
    }

    pub async fn send_listen(&mut self, mut message: JsonRpcRequest) -> Result<JsonRpcResponse, Box<dyn std::error::Error>> {
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
        message.id = id;

        let (tx, rx) = oneshot::channel();
        self.pending_requests.lock().await.insert(id, (tx, PendingRequestStatus::Clean));

        self.send(message).await?; // Send the message

        match tokio::time::timeout(DEFAULT_SEND_LISTEN_TIMEOUT, rx).await { // Example timeout of 5 seconds
            Ok(Ok(response)) => Ok(response),
            Ok(Err(_)) => Err("Channel closed unexpectedly".to_string().into()),
            Err(_) => Err("Timeout waiting for response".to_string().into()),
        }
    }
}

// use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};
// use serde::{Serialize, Deserialize};
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

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// enum JsonRpcVersion {
//     #[serde(rename = "2.0")]
//     V2
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct JsonRpcRequest {
//     /// The JSON-RPC version of the request.
//     jsonrpc: JsonRpcVersion,
//     /// The method to be called.
//     pub method: String,
//     /// The parameters to be passed to the method.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub params: Option<serde_json::Value>,
//     /// The ID of the request.
//     id: u32,
// }

// impl<'a> JsonRpcRequest {
//     pub fn build(method: impl ToString, params: Option<serde_json::Value>) -> Self {
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             method: method.to_string(),
//             params,
//             id: 0,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum JsonRpcResponse {
//     Result(serde_json::Value),
//     ReturnedError(serde_json::Value),
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcError {
//     code: i32,
//     message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcErrorResponse {
//     error: JsonRpcError,
//     id: u32,
//     jsonrpc: JsonRpcVersion,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcSuccessResponse {
//     result: serde_json::Value,
//     id: u32,
//     jsonrpc: JsonRpcVersion,
// }

// pub struct JsonRpcWsClient {
//     /// The sender for sending messages to the WebSocket server.
//     ws_writer: mpsc::Sender<JsonRpcRequest>,
//     /// The receiver for receiving messages from the WebSocket server.
//     ws_reader: mpsc::Receiver<JsonRpcResponse>,
//     /// The sender for sending shutdown signals to the WebSocket server.
//     shutdown_sender: mpsc::Sender<()>,
//     /// A counter for generating unique IDs for messages.
//     id_counter: AtomicUsize,
//     /// A map of pending requests, where the key is the message ID and the value is a sender for sending the response.
//     pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<JsonRpcResponse>>>>,
// }

// impl Drop for JsonRpcWsClient {
//     fn drop(&mut self) {
//         let shutdown_sender = self.shutdown_sender.clone();
//         tokio::spawn(async move {
//             shutdown_sender.send(()).await.ok();
//         });
//     }
// }

// impl JsonRpcWsClient {
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

//         let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<JsonRpcResponse>>::new()));
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
//                                 let raw_value: serde_json::Value = serde_json::from_str(&message_txt).unwrap();
//                                 let parsed: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
//                                 match parsed {
//                                     Ok(wrapped_message) => {
//                                         let id = wrapped_message.id;
//                                         if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                             tx.send(JsonRpcResponse::Result(wrapped_message.result)).ok();
//                                         } else {
//                                             ws_reader_sender.send(JsonRpcResponse::Result(wrapped_message.result)).await.ok();
//                                         }
//                                     },
//                                     Err(_) => {
//                                         let parsed: Result<JsonRpcErrorResponse, serde_json::Error> = serde_json::from_str(&message_txt);
//                                         match parsed {
//                                             Ok(wrapped_message) => {
//                                                 let id = wrapped_message.id;
//                                                 if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                                     tx.send(JsonRpcResponse::ReturnedError(serde_json::Value::Null)).ok();
//                                                 } else {
//                                                     ws_reader_sender.send(JsonRpcResponse::ReturnedError(serde_json::Value::Null)).await.ok();
//                                                 }
//                                             },
//                                             Err(_) => {
//                                                 tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
//                                                 tracing::error!("Message Length: {}", message_txt.len());
//                                                 tracing::error!("{}", message_txt);
//                                                 tracing::error!("--------------------------------------------------------------------------");
//                                             },
//                                         }
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

//     pub async fn recv(&mut self) -> Option<JsonRpcResponse> {
//         match self.ws_reader.recv().await {
//             Some(message) => Some(message),
//             None => None,
//         }
//     }

//     pub async fn send(&mut self, mut message: JsonRpcRequest) -> Result<(), SendError<JsonRpcRequest>> {
//         self.ws_writer.send(message).await?;
//         Ok(())
//     }

//     pub async fn send_listen(&mut self, mut message: JsonRpcRequest) -> Result<JsonRpcResponse, Box<dyn std::error::Error>> {
//         let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
//         message.id = id;

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



// use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};
// use serde::{Serialize, Deserialize};
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

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// enum JsonRpcVersion {
//     #[serde(rename = "2.0")]
//     V2
// }

// // #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// // struct JsonRpcRequest<'a> {
// //     /// The JSON-RPC version of the request.
// //     jsonrpc: JsonRpcVersion,
// //     /// The method to be called.
// //     pub method: String,
// //     /// The parameters to be passed to the method.
// //     #[serde(skip_serializing_if = "Option::is_none")]
// //     pub params: Option<&'a serde_json::value::RawValue>,
// //     /// The ID of the request.
// //     id: u32,
// // }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcRequest<'a> {
//     /// The JSON-RPC version of the request.
//     jsonrpc: JsonRpcVersion,
//     /// The method to be called.
//     pub method: String,
//     /// The parameters to be passed to the method.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub params: Option(serde_json::Value),
//     /// The ID of the request.
//     id: u32,
// }

// impl<'a> JsonRpcRequest<'a> {
//     fn build(method: impl ToString, params: Option<&'a serde_json::value::RawValue>) -> Self {
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             method: method.to_string(),
//             params,
//             id: 0,
//         }
//     }
// }

// // #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum JsonRpcResponse<'a> {
//     Result(&'a serde_json::value::RawValue),
//     ReturnedError(&'a serde_json::value::RawValue),
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcError {
//     code: i32,
//     message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcErrorResponse {
//     error: JsonRpcError,
//     id: u32,
//     jsonrpc: JsonRpcVersion,
// }

// // #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// // #[derive(Debug, Clone, Serialize, Deserialize)]
// // struct JsonRpcSuccessResponse<'a> {
// //     result: &'a serde_json::value::RawValue,
// //     id: u32,
// //     jsonrpc: JsonRpcVersion,
// // }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct JsonRpcSuccessResponse {
//     result: serde_json::Value,
//     id: u32,
//     jsonrpc: JsonRpcVersion,
// }

// pub struct JsonRpcWsClient {
//     /// The sender for sending messages to the WebSocket server.
//     ws_writer: mpsc::Sender<JsonRpcRequest<'static>>,
//     /// The receiver for receiving messages from the WebSocket server.
//     ws_reader: mpsc::Receiver<JsonRpcResponse<'static>>,
//     /// The sender for sending shutdown signals to the WebSocket server.
//     shutdown_sender: mpsc::Sender<()>,
//     /// A counter for generating unique IDs for messages.
//     id_counter: AtomicUsize,
//     /// A map of pending requests, where the key is the message ID and the value is a sender for sending the response.
//     pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<JsonRpcResponse<'static>>>>>,
// }

// impl Drop for JsonRpcWsClient {
//     fn drop(&mut self) {
//         let shutdown_sender = self.shutdown_sender.clone();
//         tokio::spawn(async move {
//             shutdown_sender.send(()).await.ok();
//         });
//     }
// }

// impl JsonRpcWsClient {
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

//         let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<JsonRpcResponse<'static>>>::new()));
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
//                                 let raw_value: serde_json::Value = serde_json::from_str(&message_txt).unwrap();
//                                 let parsed: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
//                                 match parsed {
//                                     Ok(wrapped_message) => {
//                                         let id = wrapped_message.id;
//                                         if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                             tx.send(JsonRpcResponse::Result(raw_value)).ok();
//                                         } else {
//                                             ws_reader_sender.send(JsonRpcResponse::Result(raw_value)).await.ok();
//                                         }
//                                     },
//                                 // let raw_value: serde_json::value::RawValue = serde_json::from_str(&message_txt).unwrap();
//                                 // let parsed: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
//                                 // match parsed {
//                                 //     Ok(wrapped_message) => {
//                                 //         let id = wrapped_message.id;
//                                 //         if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                 //             tx.send(JsonRpcResponse::Result(&raw_value)).ok();
//                                 //         } else {
//                                 //             ws_reader_sender.send(JsonRpcResponse::Result(&raw_value)).await.ok();
//                                 //         }
//                                 //     },
//                                     Err(_) => {
//                                         let parsed: Result<JsonRpcErrorResponse, serde_json::Error> = serde_json::from_str(&message_txt);
//                                         match parsed {
//                                             Ok(wrapped_message) => {
//                                                 let id = wrapped_message.id;
//                                                 if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                                     tx.send(JsonRpcResponse::ReturnedError(&raw_value)).ok();
//                                                 } else {
//                                                     ws_reader_sender.send(JsonRpcResponse::ReturnedError(&raw_value)).await.ok();
//                                                 }
//                                             },
//                                             Err(_) => {
//                                                 tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
//                                                 tracing::error!("Message Length: {}", message_txt.len());
//                                                 tracing::error!("{}", message_txt);
//                                                 tracing::error!("--------------------------------------------------------------------------");
//                                             },
//                                         }
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

//     pub async fn recv(&mut self) -> Option<JsonRpcResponse<'static>> {
//         match self.ws_reader.recv().await {
//             Some(message) => Some(message),
//             None => None,
//         }
//     }

//     pub async fn send(&mut self, mut message: JsonRpcRequest<'static>) -> Result<(), SendError<JsonRpcRequest<'static>>> {
//         self.ws_writer.send(message).await?;
//         Ok(())
//     }

//     pub async fn send_listen(&mut self, mut message: JsonRpcRequest<'static>) -> Result<JsonRpcResponse<'static>, Box<dyn std::error::Error>> {
//         let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
//         message.id = id;

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



// use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};
// use serde::{Serialize, de::DeserializeOwned, Deserialize, Deserializer, Serializer};
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

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// enum JsonRpcVersion {
//     #[serde(rename = "2.0")]
//     V2
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcRequest {
//     /// The JSON-RPC version of the request.
//     jsonrpc: JsonRpcVersion,
//     /// The method to be called.
//     pub method: String,
//     /// The parameters to be passed to the method.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub params: Option<serde_json::value::RawValue>,
//     /// The ID of the request.
//     id: u32,
// }

// impl JsonRpcRequest {
//     fn build(method: impl ToString, params: Option<serde_json::value::RawValue>) -> Self {
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             method: method.to_string(),
//             params,
//             id: 0,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum JsonRpcResponse {
//     Result(serde_json::value::RawValue),
//     ReturnedError(serde_json::value::RawValue),
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcError {
//     code: i32,
//     message: String,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcErrorResponse {
//     error: JsonRpcError,
//     id: u32,
//     jsonrpc: JsonRpcVersion,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcSuccessResponse {
//     result: serde_json::value::RawValue,
//     id: u32,
//     jsonrpc: JsonRpcVersion,
// }

// pub struct JsonRpcWsClient {
//     /// The sender for sending messages to the WebSocket server.
//     ws_writer: mpsc::Sender<JsonRpcRequest>,
//     /// The receiver for receiving messages from the WebSocket server.
//     ws_reader: mpsc::Receiver<JsonRpcResponse>,
//     /// The sender for sending shutdown signals to the WebSocket server.
//     shutdown_sender: mpsc::Sender<()>,
//     /// A counter for generating unique IDs for messages.
//     id_counter: AtomicUsize,
//     /// A map of pending requests, where the key is the message ID and the value is a sender for sending the response.
//     pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<JsonRpcResponse>>>>,
// }

// impl Drop for JsonRpcWsClient {
//     fn drop(&mut self) {
//         let shutdown_sender = self.shutdown_sender.clone();
//         tokio::spawn(async move {
//             shutdown_sender.send(()).await.ok();
//         });
//     }
// }

// impl JsonRpcWsClient {
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

//         let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<JsonRpcResponse>>::new()));
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
//                                 let parsed: Result<JsonRpcSuccessResponse, serde_json::Error> = serde_json::from_str(&message_txt);
//                                 match parsed {
//                                     Ok(wrapped_message) => {
//                                         let raw_value = wrapped_message.result;
//                                         let id = wrapped_message.id;
//                                         if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                             tx.send(JsonRpcResponse::Result(raw_value)).ok();
//                                         } else {
//                                             ws_reader_sender.send(JsonRpcResponse::Result(raw_value)).await.ok();
//                                         }
//                                     },
//                                     Err(_) => {
//                                         let parsed: Result<JsonRpcErrorResponse, serde_json::Error> = serde_json::from_str(&message_txt);
//                                         match parsed {
//                                             Ok(wrapped_message) => {
//                                                 let raw_value = serde_json::value::RawValue::from_string(serde_json::to_string(&wrapped_message.error).unwrap());
//                                                 let id = wrapped_message.id;
//                                                 if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                                     tx.send(JsonRpcResponse::ReturnedError(raw_value)).ok();
//                                                 } else {
//                                                     ws_reader_sender.send(JsonRpcResponse::ReturnedError(raw_value)).await.ok();
//                                                 }
//                                             },
//                                             Err(_) => {
//                                                 tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
//                                                 tracing::error!("Message Length: {}", message_txt.len());
//                                                 tracing::error!("{}", message_txt);
//                                                 tracing::error!("--------------------------------------------------------------------------");
//                                             },
//                                         }
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

//     pub async fn recv(&mut self) -> Option<JsonRpcResponse> {
//         match self.ws_reader.recv().await {
//             Some(message) => Some(message),
//             None => None,
//         }
//     }

//     pub async fn send(&mut self, mut message: JsonRpcRequest) -> Result<(), SendError<JsonRpcRequest>> {
//         self.ws_writer.send(message).await?;
//         Ok(())
//     }

//     pub async fn send_listen(&mut self, mut message: JsonRpcRequest) -> Result<JsonRpcResponse, Box<dyn std::error::Error>> {
//         let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
//         message.id = id;

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






// use std::sync::{atomic::AtomicUsize, atomic::Ordering, Arc};
// use serde::{Serialize, de::DeserializeOwned, Deserialize, Deserializer, Serializer};
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

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// enum JsonRpcVersion {
//     #[serde(rename = "2.0")]
//     V2
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcRequest<'a> {
//     /// The JSON-RPC version of the request.
//     jsonrpc: JsonRpcVersion,
//     /// The method to be called.
//     pub method: String,
//     /// The parameters to be passed to the method.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub params: Option<serde_json::value::RawValue>,
//     /// The ID of the request.
//     id: u32,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct JsonRpcResponse {
//     /// The JSON-RPC version of the response.
//     jsonrpc: JsonRpcVersion,
//     /// The result of the request.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub result: Option<serde_json::value::RawValue>,
//     /// The error of the request.
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub error: Option<serde_json::value::RawValue>,
//     /// The ID of the response.
//     id: u32,
// }

// #[derive(Debug, Clone)]
// pub enum Response {
//     Result(serde_json::value::RawValue),
//     Error(serde_json::value::RawValue),
// }

// #[derive(Debug, Clone)]
// pub enum Message {
//     Request(JsonRpcRequest),
//     Response(JsonRpcResponse),
//     Notification(JsonRpcRequest),
// }

// pub struct JsonRpcWsClient {
//     /// The sender for sending messages to the WebSocket server.
//     ws_writer: mpsc::Sender<JsonRpcRequest>,
//     /// The receiver for receiving messages from the WebSocket server.
//     ws_reader: mpsc::Receiver<Message>,
//     /// The sender for sending shutdown signals to the WebSocket server.
//     shutdown_sender: mpsc::Sender<()>,
//     /// A counter for generating unique IDs for messages.
//     id_counter: AtomicUsize,
//     /// A map of pending requests, where the key is the message ID and the value is a sender for sending the response.
//     pending_requests: Arc<Mutex<HashMap<u32, oneshot::Sender<Response>>>>,
// }

// impl JsonRpcWsClient {
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
        
//         let pending_requests = Arc::new(Mutex::new(HashMap::<u32, oneshot::Sender<Response>>::new()));
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
//                                 let parsed: JsonRpcResponse = match serde_json::from_str(&message_txt) {
//                                     Ok(res) => res,
//                                     Err(_) => {
//                                         let parsed: JsonRpcRequest = match serde_json::from_str(&message_txt) {
//                                             Ok(req) => req,
//                                             Err(_) => {
//                                                 tracing::error!("----------------------------MESSAGE NOT PARSED----------------------------");
//                                                 tracing::error!("Message Length: {}", message_txt.len());
//                                                 tracing::error!("{}", message_txt);
//                                                 tracing::error!("--------------------------------------------------------------------------");
//                                                 continue;
//                                             }
//                                         };
//                                         ws_reader_sender.send(Message::Notification(parsed)).await.ok();
//                                         continue;
//                                     }
//                                 };
//                                 match parsed.id {
//                                     id => {
//                                         if let Some(tx) = pending_requests_clone.lock().await.remove(&id) {
//                                             let response = match (parsed.result, parsed.error) {
//                                                 (Some(res), None) => Response::Result(res),
//                                                 (None, Some(err)) => Response::Error(err),
//                                                 _ => panic!("Invalid response"),
//                                             };
//                                             tx.send(response).ok();
//                                         } else {
//                                             ws_reader_sender.send(Message::Response(parsed)).await.ok();
//                                         }
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

//     pub async fn recv(&mut self) -> Option<Message> {
//         match self.ws_reader.recv().await {
//             Some(message) => Some(message),
//             None => None,
//         }
//     }

//     pub async fn send(&mut self, request: JsonRpcRequest) -> Result<(), SendError<JsonRpcRequest>> {
//         self.ws_writer.send(request).await?;
//         Ok(())
//     }

//     pub async fn send_listen(&mut self, mut request: JsonRpcRequest) -> Result<Response, Box<dyn std::error::Error>> {
//         let id = self.id_counter.fetch_add(1, Ordering::SeqCst) as u32;
//         request.id = id;

//         let (tx, rx) = oneshot::channel();
//         self.pending_requests.lock().await.insert(id, tx);

//         self.send(request).await?; // Send the request

//         match tokio::time::timeout(DEFAULT_SEND_LISTEN_TIMEOUT, rx).await { // Example timeout of 5 seconds
//             Ok(Ok(response)) => Ok(response),
//             Ok(Err(_)) => Err("Channel closed unexpectedly".to_string().into()),
//             Err(_) => Err("Timeout waiting for response".to_string().into()),
//         }
//     }
// }

// impl Drop for JsonRpcWsClient {
//     fn drop(&mut self) {
//         let shutdown_sender = self.shutdown_sender.clone();
//         tokio::spawn(async move {
//             shutdown_sender.send(()).await.ok();
//         });
//     }
// }

// pub fn build_request(method: impl ToString, params: Option<serde_json::value::RawValue>) -> JsonRpcRequest {
//     JsonRpcRequest {
//         jsonrpc: JsonRpcVersion::V2,
//         method: method.to_string(),
//         params,
//         id: 0,
//     }
// }