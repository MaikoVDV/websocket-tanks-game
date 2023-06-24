use crate::networking::*;
use tokio_tungstenite::tungstenite::Message;

use futures_util::stream::SplitSink;

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
