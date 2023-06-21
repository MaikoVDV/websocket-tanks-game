use crate::*;

pub enum ConnectionEvent {
    Connected,
    Disconnected,
    Error,
}