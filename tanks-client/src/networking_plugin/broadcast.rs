use crate::networking_plugin::*;

pub async fn broadcast(
    ws_sender: SplitSink<WebSocketStream<TlsStream<TcpStream>>, Message>,
) {
    info!("Ready to broadcast messages to the server");
}