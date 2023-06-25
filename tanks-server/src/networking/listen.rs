use crate::*;

/// Listen for incoming data from clients. Runs in a separate tokio task for each client.
pub async fn listen(
    event_sender: mpsc::UnboundedSender<ClientEvents>,
    ws_stream: WebSocketStream<TlsStream<TcpStream>>,
    connections: Arc<DashMap<u32, ClientConnection>>,
    id: u32,
) {
    let (sender, mut receiver) = ws_stream.split();
    let conn = ClientConnection::new(id, sender);

    let _ = event_sender.send(ClientEvents::Connected(conn.id));
    connections.insert(conn.id, conn);
    while let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if msg.is_binary() {
                //info!("Received message: {}", msg.to_string());
                let mut msg = msg.into_data();
                if msg.len() <= 0 {
                    error!("Received a message with a length of 0 or less. Not processing.");
                    continue;
                }
                let header = msg.remove(0);
                // let mut reader = BytesReader::from_bytes(&msg);
                match header {
                    20 => {
                        // if let Ok(input) = generic_protobufs::ClientInput::from_reader(&mut reader, &msg) {
                        //     info!(
                        //        "Received the following GameInput from client {}:\nx: {}, y: {}, pressed: {}",
                        //        id, input.x, input.y, input.pressed
                        //     );
                        //     let _ = event_sender.send(ClientEvents::Input(id, input));
                        // }
                    }
                    _ => ()
                }
            } else if msg.is_close() {
                break; // When we break, we disconnect.
            }
        } else {
            // receiver.next returned None, meaning the stream was closed, and the cliend disconnected.
            break; // When we break, we disconnect.
        }
    }
    
    // Send quit event to game loop, and the game loop will send quit event to the broadcast thread.
    info!("Client {} disconnected.", id);
    event_sender.send(ClientEvents::Disconnected(id)).expect("Failed to send ClientEvents::Quit(id)");
}
