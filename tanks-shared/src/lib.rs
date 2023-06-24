use tokio::sync::mpsc;

use std::{
    time,
    collections::HashMap,
};

pub mod proto_compiled;
use proto_compiled::{
    network_messages::*,
    entities::*,
};

pub mod game_manager;

// The interval at which the game loop runs.
const TICKS_PER_SECOND: u64 = 20;

#[tokio::main]
async fn main() {
    println!("Shared library loaded.");
}
