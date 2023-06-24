use crate::{
    *,
    networking_plugin::*,
};

/// Stores and manages data about the current connection between the client and server.
/// Most importantly, stores JoinHandles for the listening and broadcasting tasks that actually
/// do the communicating with the server.
pub struct ServerConnection {
    pub server_url: Url,
    pub ws_stream: WebSocketStream<TlsStream<TcpStream>>,

    listen_task: JoinHandle<()>,
    broadcast_task: JoinHandle<()>,
}
impl ServerConnection {
    pub fn new(
        server_url: Url,
        ws_stream: WebSocketStream<TlsStream<TcpStream>>,
        listen_task: JoinHandle<()>,
        broadcast_task: JoinHandle<()>
    ) -> ServerConnection {
        info!("Connected to server!");
        ServerConnection {
            server_url,
            ws_stream,
            listen_task,
            broadcast_task,
        }
    }
    /// Kill the listening and broadcasting threads.
    pub fn disconnect(self) {
        info!("Disconnecting and aborting networking tasks.");
        self.listen_task.abort();
        self.broadcast_task.abort();
    }
}
