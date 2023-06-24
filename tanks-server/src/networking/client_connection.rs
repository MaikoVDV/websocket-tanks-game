use crate::networking::*;

/// Stores data about the connection with a client.
/// The 'sender' is a WebsocketStream (and the streams within) split apart to only include the Sink part.
/// Used only for *sending* data *to* the client.
pub struct ClientConnection {
    pub id: u32,
    pub sender: SplitSink<WebSocketStream<TlsStream<TcpStream>>, Message>,
}

impl ClientConnection {
    pub fn new(id: u32, sender: SplitSink<WebSocketStream<TlsStream<TcpStream>>, Message>) -> ClientConnection {
        ClientConnection {
            id,
            sender,
        }
    }
}
