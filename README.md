# moonsock

Moonsock is a simple way to connect to a klipper/moonraker 3D printer websocket from anywhere where tokio can be ran and your printer is accessible from the internet.

# Usage

```rust
// I prefer starting my own tokio runtimes for this sort of use-case. 
println!("Starting Main Runtime");
let moon_rt = Builder::new_multi_thread()
    .worker_threads(1)
    .thread_name("Moonraker Websocket Main")
    .enable_all()
    .build()
    .unwrap();
// Note: the port might be different for you, but 7125 is moonraker's default port. Check your moonraker.conf if you have problems
let url = "ws://[printer ip or hostname]:7125/websocket";
let mut connection = moon_rt.block_on(MoonConnection::new(URL.to_string(), 1000, 1000));

// This part below needs to be ran in a tokio runtime. Although we do have support for blocking calls on connection for both sending and receiving messages.
match connection.recv().await {
    Some(msg) => println!("Received: {:?}", msg),
    None => println!("Failed to receive message from connection"),
}

let temp_id = 42342;
let message = MoonMSG::new(MoonMethod::ServerTemperatureStore, None, Some(temp_id));
// This will send a message and return the response for that specific message. An id should be set on the sent message or else the future might never yield. 
match connection.send_listen(message.clone()).await {
    Ok(message) => {
        ...
    },
    Err(_) => continue,
}
```

# Completeness
Not all message types are supported by the parser currently, but most of the important ones are. If you want support for more messages, either wait or create a pull request on the github.
