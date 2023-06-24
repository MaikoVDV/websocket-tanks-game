use crate::{
    *,
    networking_plugin::*
};

#[derive(Resource)]
pub struct WebsocketClient {
  // Put all the networking code on a separate thread to prevent blocking.
  pub tokio_runtime: Runtime,
  // Holds info about the connection. Is None if not connected / in-game.
  pub server_connection: Option<ServerConnection>,

  pub events_channel: SyncChannel<ConnectionEvent>,
    pub debug: u16,
}


pub use native_tls::TlsConnector;
pub use tokio_native_tls::TlsConnector as TokioTlsConnector;


impl WebsocketClient {
    pub fn new() -> WebsocketClient {
        WebsocketClient {
            // Prepare the runtime for handling listening and broadcasting tasks.
            tokio_runtime: runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to build tokio runtime for websocket client."),
            server_connection: None,
            events_channel: SyncChannel::new(),
            debug: 69,
        }
    }

    pub fn connect(&mut self, server_url: Url) {
        info!("Connecting to websocket at {}", server_url);
        self.disconnect();




        // Channel used to send the ServerConnection from the connection task to the main task,
        // to put in the WebsocketClient.
        let (stream_tx, mut stream_rx) = oneshot::channel();
        self.tokio_runtime.spawn(async move {
            //let websocket_url = &addr;
            // info!("{}", websocket_url);

            // Create a TCP connection with the server
            let tcp_stream = TcpStream::connect("127.0.0.1:443").await.expect("Failed to connect");
            // Using TCP stream to create websocket and upgrading connetion with TLS (making it encrypted)
            let tls_connector = TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap();
            let tls_connector = TokioTlsConnector::from(tls_connector);
            let tls_stream = tls_connector.connect("wss://127.0.0.1:443", tcp_stream).await.expect("Failed to connect with TLS.");

            let (mut ws_stream, _) = tokio_tungstenite::client_async("wss://127.0.0.1:443", tls_stream)
                .await
                .expect("Failed to create websocket.");

            // Connecting to the websocket.
            // let ws_stream = match connect_async(url.clone()).await {
            //     Ok((ws_stream, _response)) => ws_stream,
            //     Err(e) => {
            //         error!("Failed to connect to server at '{}'. Here's the error: {}", url.to_string(), e.to_string());
            //         return;
            //     }
            // };
            
            // let server_conn = ServerConnection::new(server_url.clone(), ws_stream);
            stream_tx.send(ws_stream).expect("Failed to send WebSocketStream through OneShot channel. Maybe it was closed / dropped?");
        });

        // Creating a Bevy task in order to modify WebsocketClient (to add a ServerConnection)
        // without messing with threads and stuff.
        let connect_task_pool = TaskPool::new();
        connect_task_pool.scope(|s| {
            s.spawn(async {
                // Connecting to the websocket.
                match stream_rx.await {
                    Ok(ws_stream) => {
                        info!("Received WebSocketStream through OneShot channel!");
                        let listen_task = self.tokio_runtime.spawn(async move {

                        });
                        let broadcast_task = self.tokio_runtime.spawn(async move {

                        });

                        self.server_connection = Some(
                            ServerConnection::new(
                                server_url,
                                ws_stream,
                                listen_task,
                                broadcast_task
                        ));
                    },
                    // Can really only be a RecvError, meaning the channel got dropped before a
                    // value was sent.
                    Err(_) => {
                        error!("Failed to receive WebsocketStream, the sender probably got dropped due to a connection error.");
                    }
                }
            })
        });
    }

    // Tell the ServerConnection to end its Tokio Tasks, and send a disconnection event internally.
    pub fn disconnect(&mut self) {
        debug!("Disconnecting from websocket.");

        if let Some(server_conn) = self.server_connection.take() {
          // server_conn.stop();

          // let _ = self
          //     .connection_events
          //     .sender
          //     .send(ConnectionEvent::Disconnected);
        }
    }
}

// Used to pass messages synchronously. Alternative to std::sync::mpsc.
pub struct SyncChannel<T> {
  pub(crate) sender: crossbeam_channel::Sender<T>,
  pub(crate) receiver: crossbeam_channel::Receiver<T>,
}

impl<T> SyncChannel<T> {
  pub fn new() -> Self {
      let (sender, receiver) = crossbeam_channel::unbounded();

      SyncChannel { sender, receiver }
  }
}
