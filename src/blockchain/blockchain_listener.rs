use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};

type SharedClients = Arc<Mutex<HashMap<String, tokio_tungstenite::WebSocketStream<TcpStream>>>>;

#[derive(Debug)]
struct WebSocketServer {
    address: String,
    clients: SharedClients,
}

impl WebSocketServer {
    async fn run(&self, broadcast_tx: broadcast::Sender<String>) {
        let listener = TcpListener::bind(&self.address)
            .await
            .expect("Failed to bind to address");
        println!("WebSocket server running at ws://{}", self.address);

        loop {
            if let Ok((stream, addr)) = listener.accept().await {
                let client_id = addr.to_string();
                println!("New connection from {}", client_id);

                let clients = Arc::clone(&self.clients);
                let tx = broadcast_tx.clone();

                tokio::spawn(async move {
                    if let Err(e) = WebSocketServer::handle_connection(stream, client_id, clients, tx).await {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
            }
        }
    }

    async fn handle_connection(
        stream: TcpStream,
        client_id: String,
        clients: SharedClients,
        broadcast_tx: broadcast::Sender<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let ws_stream = accept_async(stream).await?;
        let (mut writer, mut reader) = ws_stream.split();

        // Add the client to the shared client list
        // {
        //     let mut clients_lock = clients.lock().unwrap();
        //     clients_lock.insert(client_id.clone(), writer.reunite(reader)?);
        // }

        // Send a welcome message
        let greeting = format!("Hi from server!");
        writer.send(Message::Text(greeting)).await?;

        // Read messages from this client and broadcast them
        while let Some(msg) = reader.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    println!("Received from {}: {}", client_id, text);
                    broadcast_tx.send(format!("{}: {}", client_id, text))?;
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error with client {}: {}", client_id, e);
                    break;
                }
            }
        }

        // Remove the client when it disconnects
        {
            let mut clients_lock = clients.lock().unwrap();
            clients_lock.remove(&client_id);
        }

        println!("Client {} disconnected", client_id);
        Ok(())
    }
}

pub struct BlockchainListener { }

impl BlockchainListener {
    pub fn run(address: String) {
    let server_address = address.to_string();

    // Shared state for managing connected clients (TODO)
    let clients: SharedClients = Arc::new(Mutex::new(HashMap::new()));

    // Broadcast channel for sending messages to all clients
    let (broadcast_tx, _broadcast_rx) = broadcast::channel(100);

    // Start the WebSocket server
    let server = WebSocketServer {
        address: server_address.clone(),
        clients: Arc::clone(&clients),
    };

    tokio::spawn(async move {
        server.run(broadcast_tx).await;
    });

    println!("Server running. Connect any WebSocket client to ws://{}", server_address);
    
    // Keep the server running indefinitely
    // tokio::signal::ctrl_c()
    //     .await
    //     .expect("Failed to listen for ctrl-c signal");
    }
}


