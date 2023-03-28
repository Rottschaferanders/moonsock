use tokio::sync::mpsc::{
    error::SendError,
    Permit,
};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use core::pin::Pin;
use futures_util::{sink::*, StreamExt};
use url::Url;
// use crate::*;

use crate::{MoonMSG, MoonResultData};

/// A WebSocket connection to a Moonraker server.
pub struct MoonConnection {
    write_stream: tokio::sync::mpsc::Sender<MoonMSG>,
    read_stream: tokio::sync::mpsc::Receiver<MoonMSG>,
}
impl MoonConnection {
    /// Creates a new `MoonConnection` instance and establishes a WebSocket connection to the specified `url`.
    ///
    /// # Arguments
    ///
    /// * `url` - A `String` containing the URL of the Moonraker instance to connect to.
    ///
    /// # Returns
    ///
    /// A new `MoonConnection` instance.
    pub async fn new(url: String, writer_buffer_size: usize, reader_buffer_size: usize) -> MoonConnection {
        let (ws_writer_sender, mut ws_writer_receiver) = tokio::sync::mpsc::channel(writer_buffer_size);
        let connect_addr = Url::parse(&url).unwrap();
        let (ws_stream, _) = match connect_async(&connect_addr).await {
            Ok(stuff) => stuff,
            Err(_) => panic!("Error connecting to websocket"),
        };
        println!("WebSocket handshake has been successfully completed");
        let (mut moon_socket_sink, mut moon_socket_stream) = ws_stream.split();

        // Spawns Moonraker Websocket writer thread
        tokio::spawn(async move {
            while let Some(msg) = ws_writer_receiver.recv().await {
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
        println!("Split websocket Stream");
        let (ws_reader_sender, ws_reader_receiver) = tokio::sync::mpsc::channel(reader_buffer_size);
        println!("Created Reader Runtime");
        // Spawns Moonraker Websocket reader thread
        tokio::spawn(async move {
            while let Some(message) = moon_socket_stream.next().await {
                match message {
                    Ok(msg) => {
                        if msg.len() == 0 {
                            continue;
                        }
                        let message = msg.into_text().unwrap();
                        println!("Received: {}", message);
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
                    Err(_) => println!("Hi, I'm a error"),
                }
            }
        });

        MoonConnection {
            write_stream: ws_writer_sender,
            read_stream: ws_reader_receiver,
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
    pub async fn send(&mut self, message: MoonMSG) -> Result<(), SendError<MoonMSG>> {
        self.write_stream.send(message).await?;
        Ok(())
    }
    
    /// Sends a message over the WebSocket connection, using a reserved spot in the writer queue.
    ///
    /// This function is similar to `send`, but it uses a reserved spot in the writer queue, so it
    /// doesn't block the thread if the queue is full, but also ensures that the order of messages is exactly as your program describes. 
    /// Use this function if you are sending a lot of messages in a short amount of time and the order of those messages matters.
    /// 
    /// Essentially, you are putting a dynmic buffer on top of the fixed-sized primary message buffer to ensure that messages are sent in the order you want.
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
    pub async fn send_reserved(&mut self, message: MoonMSG) -> Result<(), SendError<()>> {
        let permit = self.reserve().await?;
        permit.send(message);
        Ok(())
    }
    /// Reserves a permit from the WebSocket writer queue for sending a message.
    ///
    /// # Returns
    ///
    /// Returns a `Permit<MoonMSG>` if a permit was successfully reserved, or a `SendError<()>` if the connection has closed.
    pub async fn reserve(&self) -> Result<Permit<MoonMSG>, SendError<()>> {
        self.write_stream.reserve().await
    }
    /// Waits for a message to be received from the Moonraker instance.
    ///
    /// # Returns
    ///
    /// Returns an `Option<MoonMSG>` containing the received message, or `None` if the receiver channel has been closed.
    pub async fn recv(&mut self) -> Option<MoonMSG> {
        self.read_stream.recv().await
    }
    pub async fn send_checked(&mut self, message: MoonMSG) -> Result<(), SendError<MoonMSG>> {
        let this_id = 3243;
        let msg = message.set_id(this_id);
        self.send(msg).await?;
        // let mut ok_received = false;
        loop {
            match self.recv().await {
                Some(msg) => {
                    match msg {
                        MoonMSG::MoonResult { id, result, .. } => {
                            if id == this_id {
                                match result {
                                    MoonResultData::Ok(..) => {
                                        return Ok(());
                                    }
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
        // Ok(())
    }
    pub async fn send_listen(&mut self, message: MoonMSG) -> Result<MoonMSG, SendError<MoonMSG>> {
        let this_id = message.id().expect("Message must have an ID");
        self.send(message).await?;
        loop {
            match self.recv().await {
                Some(msg) => {
                    match msg.id() {
                        Some(id) => {
                            if id == this_id {
                                return Ok(msg)
                            }
                        },
                        None => continue,
                    }
                },
                None => continue,
            }
        }
        // Ok(())
    }
}