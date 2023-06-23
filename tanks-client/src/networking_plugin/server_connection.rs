use crate::*;

pub struct ServerConnection {
    pub server_url: Url,
    pub ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,

    listen_task: JoinHandle<()>,
    broadcast_task: JoinHandle<()>,
}
impl ServerConnection {
    pub fn new(
        server_url: Url,
        ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
        listen_task: JoinHandle<()>,
        broadcast_task: JoinHandle<()>
    ) -> ServerConnection {

        ServerConnection {
            server_url,
            ws_stream,
            listen_task,
            broadcast_task,
        }
    }
    // Kill the listening and broadcasting threads.
    pub fn disconnect(self) {
        self.listen_task.abort();
        self.broadcast_task.abort();
    }
}
