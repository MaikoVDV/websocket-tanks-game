use crate::networking_plugin::*;

use tanks_shared::{
    game_manager::proto_serialization::quick_protobuf,
    proto_compiled::state as proto_state,
};

/// Listen for messages from the server.
/// Messages are serialized as ProtoBufs, so they need to be deserialized based on their type.
pub async fn listen(
    mut ws_receiver: SplitStream<WebSocketStream<TlsStream<TcpStream>>>,
) {
    info!("Listening for messages from the server...");
    while let Some(Ok(received_data)) = ws_receiver.next().await {
        match received_data {
            Message::Binary(mut bin_msg) => {
                if bin_msg.len() <= 0 {
                    info!("Received a message with a length of 0 or less. Not processing.");
                    continue;
                }

                // Messages are prefixed with a header byte. Specifies the type of message, so it can be deserialized.
                let header = bin_msg.remove(0);

                match header {
                    10 => { // State update
                        if let Ok(state_update) = quick_protobuf::deserialize_from_slice::<proto_state::StateUpdate>(&bin_msg) {
                            // info!("{:?}", state_update);
                        }
                    }
                    _ => {
                        warn!("Received a message with an unknown header ({}) Ignoring.", header);
                        continue;
                    }
                }
            }
            _ => {
                warn!("Received a non-binary message");
            }
        }
    }
}