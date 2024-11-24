use crate::websockets::WebSocketServer;

pub struct BlockchainListener {}

impl BlockchainListener {
    pub async fn run(address: String) {

        let server = WebSocketServer::new();

        // Run the WebSocket server as a separate task
        tokio::spawn(async move {
            if let Err(e) = server.run(&address, Self::on_client_message).await {
                eprintln!("Error running server: {}", e);
            }
        });

        // tokio::spawn(async {
        //     tokio::signal::ctrl_c().await.unwrap();
        //     println!("Server stopped.");
        // });

        // Continue with other tasks that need to run concurrently
        println!("Websocket initialized...");
    }

    async fn on_client_message(msg: String) {
        //println!("{} {}", "Handling message: ", msg);
        // Process the message (e.g., log, respond, etc.)
    }
}
