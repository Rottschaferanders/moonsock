// use fastwebsockets::handshake;
// use fastwebsockets::FragmentCollector;
// use fastwebsockets::WebSocket;
// use hyper::{Request, body::Bytes, upgrade::Upgraded, header::{UPGRADE, CONNECTION}};
// use http_body_util::Empty;
// use hyper_util::rt::TokioIo;
// use tokio::net::TcpStream;
// use std::future::Future;
// use anyhow::Result;

// // pub async fn connect(host: String, port: u16) -> Result<FragmentCollector<TokioIo<Upgraded>>> {
// pub async fn connect(hostname: String, port: u16) -> Result<WebSocket<TokioIo<Upgraded>>> {
// // pub async fn connect<T>(host: String, port: u16) -> Result<WebSocket<TokioIo<T>>> {
//     println!("Trying to connect to TcpStream...");
//     // let stream = TcpStream::connect("localhost:9001").await?;
//     let key = fastwebsockets::handshake::generate_key();
//     let url = format!("ws://{hostname}:{port}/websocket");
//     let stream = TcpStream::connect(format!("{hostname}:{port}")).await?;
//     // let stream = TcpStream::connect(&url).await?;
//     println!("Got connected to TcpStream!");
//     let req = Request::builder()
//         .method("GET")
//         // .uri("http://localhost:9001/")
//         // .uri(format!("http://{host}:{port}/"))
//         // .uri(format!("ws://{host}:{port}/websocket"))
//         .uri(&url)
//         // .header("Host", "localhost:9001")
//         .header("Host", format!("{hostname}:{port}"))
//         .header(UPGRADE, "websocket")
//         .header(CONNECTION, "upgrade")
//         .header(
//         "Sec-WebSocket-Key",
//         // fastwebsockets::handshake::generate_key()
//         key,
//         )
//         .header("Sec-WebSocket-Version", "13")
//         .body(Empty::<Bytes>::new());

//     let req = match req {
//         Ok(req) => req,
//         Err(e) => {
//             println!("Error while building upgrade request");
//             return Err(e.into());
//         },
//     };


//     let (ws, response) = handshake::client(&SpawnExecutor, req, stream).await?;
//     println!("First Response: {:?}", response.body());
//     // Ok(FragmentCollector::new(ws))
//     Ok(ws)
// }


use fastwebsockets::handshake;
use fastwebsockets::WebSocket;
// use hyper::client::conn;
use hyper::{Request, body::Bytes, upgrade::Upgraded, header::{UPGRADE, CONNECTION}};
use hyper_util::rt::TokioIo;
use http_body_util::Empty;
use tokio::net::TcpStream;
use std::future::Future;
use anyhow::Result;
use url::Url;

use crate::MoonRequest;

// async fn connect(hostname: String, port: u16) -> Result<WebSocket<Upgraded>> {
pub async fn connect(hostname: String, port: u16) -> Result<WebSocket<TokioIo<Upgraded>>> {
    let url = format!("ws://{hostname}:{port}/websocket");
    let connect_addr = Url::parse(&url).unwrap();
    let domain = connect_addr.domain().unwrap();
    let port = connect_addr
        // .uri()
        // .port_u16()
        .port()
        // .or_else(|| match request.uri().scheme_str() {
        .or_else(|| match connect_addr.scheme() {
            // Some("wss") => Some(443),
            // Some("ws") => Some(80),
            "wss" => Some(443),
            "ws" => Some(80),
            _ => None,
        }).expect("Failed to figure out what port you wanted");
        // .ok_or(Error::Url(UrlError::UnsupportedUrlScheme))?;

    let addr = format!("{domain}:{port}");

    let stream = TcpStream::connect(addr).await.unwrap();
    // let stream = TcpStream::connect(format!("{hostname}:{port}")).await?;

    let server_info_req = MoonRequest::new(crate::MoonMethod::ServerInfo, None);
    let _msg_id = server_info_req.id.clone();
    let msg_str = serde_json::to_string(&server_info_req).unwrap();

    let req = Request::builder()
        .method("GET")
        // .uri("http://localhost:9001/")
        // .uri(format!("http://{hostname}:{port}/websocket"))
        // .uri(format!("http://{hostname}:{port}/ws"))
        // .uri(format!("ws://{hostname}:{port}/ws"))
        // .uri(format!("ws://{hostname}:{port}/websocket"))
        // .uri(url)
        .uri("/websocket")
        // .uri("http://scanhead.local/")
        // .header("Host", "localhost:9001")
        // .header("Host", format!("{hostname}:{port}"))
        .header("Host", hostname)
        // .header("")
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "upgrade")
        // .header("User-Agent", "")
        .header(
            "Sec-WebSocket-Key",
            fastwebsockets::handshake::generate_key(),
        )
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new()).unwrap();
        // .body(msg_str).unwrap();

    let (ws, _) = handshake::client(&SpawnExecutor, req, stream).await.unwrap();

    println!("Websocket Succesfully connected!");
    Ok(ws)
}

// Tie hyper's executor to tokio runtime
struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
  Fut: Future + Send + 'static,
  Fut::Output: Send + 'static,
{
  fn execute(&self, fut: Fut) {
    tokio::task::spawn(fut);
  }
}
