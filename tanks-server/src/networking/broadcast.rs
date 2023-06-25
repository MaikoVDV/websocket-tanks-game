use crate::{
    *,
    networking::*
};

/// Function runs on *one* separate tokio task, and broadcasts data to all connected clients.
pub async fn broadcast(
    mut broadcast_event_rx: mpsc::UnboundedReceiver<BroadcastEvents>,
    connections: Arc<DashMap<u32, ClientConnection>>,
) {
    // Wait to receive BroadcastEvents, and perform different kinds of broadcasts based on the type of BroadcastEvent
    while let Some(broadcast_event) = broadcast_event_rx.recv().await {
        match broadcast_event {
            BroadcastEvents::StateUpdate => {
                // Serializing the StateUpdate
                let state_test = proto_state::StateUpdate {
                    test: "Hello, world!".to_owned()
                };
                let data = proto_serialize(state_test, 10);

                // Looping over every ClientConnection in the DashMap, and sending StateUpdates to them.
                for mut conn_ref in connections.iter_mut() {
                    let conn: &mut ClientConnection = conn_ref.value_mut();
                    info!("Sending state update to client {}", conn.id);

                    let _ = conn.sender.send(Message::Binary(data.clone())).await;
                }
            }
            BroadcastEvents::ClientConnected(_client_id) => {
            }
            BroadcastEvents::ClientDisconnected(client_id) => {
                connections.remove(&client_id);
            }
        }
    }
    info!("Broadcasting loop has ended, which means the event channel was closed");
}