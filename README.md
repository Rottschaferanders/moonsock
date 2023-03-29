# moonsock

Moonsock is a simple way to connect to a klipper/moonraker 3D printer websocket from anywhere where tokio can be ran and your printer is accessible from the internet.

# Usage

## Connect to a printer
Open up the websocket creating a new MoonConnection inside a tokio runtime.  

```rust
// I prefer starting my own tokio runtimes for this sort of use-case. 
 use tokio::runtime::Builder;
 use moonsock::{MoonConnection, MoonMSG, MoonMethod};
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
```

## Basic Send Message
Note: The below examples need to be ran in a tokio runtime. 
```rust
let id = 4242;
connection.send(MoonMSG::gcode(
    String::from("G90"),
    id,
)).await.unwrap();

// Using our moon_rt from above it could be called like this:

moon_rt.block_on(connection.send(MoonMSG::gcode(
    String::from("G90"),
    id,
)).await.unwrap());
```

## Send Message and Wait for It's Response
The `send_listen` method is async so it doesn't block the thread while waiting.
This will send a message and return the response for that specific message. To be clear, it does not return the next received response, it returns the first response that has an id that matches the id of the sent message. An id should be set on the sent message or else the future might never yield. 

```rust
let temp_id = 42342;
let message = MoonMSG::new(MoonMethod::ServerTemperatureStore, None, Some(temp_id));
match connection.send_listen(message.clone()).await {
    Ok(message) => {
        ...
    },
    Err(err) => println!("ERROR: {}", err),
}
```

## Receive Messages
```rust
match connection.recv().await {
    Some(msg) => println!("Received: {:?}", msg),
    None => println!("Failed to receive message from connection"),
}
```

# Completeness of the crate
Not all message types are supported by the parser currently, but most of the important ones are. If you want support for more messages, either wait or create a pull request on the github.


