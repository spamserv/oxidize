use std::{error::Error, sync::Arc};

use colored::Colorize;
use futures_util::SinkExt;
use tokio::{
    net::TcpStream,
    sync::Mutex,
};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    pub ws_stream: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl WebSocketClient {
    pub async fn new(address: String) -> Result<Self, Box<dyn Error>> {
        let url_string = format!("ws://{address}");

        // Connect to the WebSocket server
        let (mut ws_stream, _) = connect_async(url_string).await?;
        println!("{}", "[Client] Connected to the server".blue());

        // Send a message to the server
        let message = "Hello, server!".to_string();
        ws_stream.send(Message::Text(message)).await?;

        Ok(Self {
            ws_stream: Arc::new(Mutex::new(ws_stream)),
        })
    }
}
