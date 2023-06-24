use crate::*;

// Sent upon receiving something from a client.
// Used to update something internally.
#[derive(Debug)]
pub enum ClientEvents {
    Connected(u32),
    Disconnected(u32),
    Input(ClientInput),
}

// Sent based on something in the GameWorld that needs to be sent to all connected clients
pub enum BroadcastEvents {

}