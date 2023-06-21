use crate::*;

#[derive(Resource)]
pub struct WebsocketClient {
  // Put all the networking code on a separate thread to prevent blocking.
  pub tokio_runtime: runtime::Runtime,
  // Holds info about the connection. Is None if not connected / in-game.
  pub server_connection: Option<ServerConnection>,

  pub events_channel: SyncChannel<ConnectionEvent>,
}
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
    }
  }

  pub fn connect(&mut self, server_url: &str) {
    info!("Connecting to websocket at {}", server_url);
    self.disconnect();

    let connect_task = self.tokio_runtime.spawn(async move {
      // Connecting to the websocket.
      // let (ws_stream, _) = connect_async(server_url)
      //     .await
      //     .expect("Failed to connect to the server");

      // return ws_stream;


      
      // // Trying to send the websocket & the address to the WebsocketClient struct for storage.
      // match created_new_connection_events.send((ws_stream, addr)) {
      //     Ok(_) => {
      //         connection_events.send(ConnectionEvent::Connected).unwrap();
      //         println!(
      //             "Successfully connected to websocket at address {}",
      //             addr.to_string()
      //         );
      //     }
      //     Err(err) => {
      //         connection_events.send(ConnectionEvent::Error).unwrap();
      //         println!("Could not initiate connection: {}", err);
      //     }
      // }
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