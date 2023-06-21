// Importing crates
use bevy::{
    prelude::*,
    log::LogPlugin,
};
use tokio::{
    runtime,
    task::JoinHandle,
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    MaybeTlsStream,
    WebSocketStream,
};
use crossbeam_channel::unbounded;

use url::Url;
// Standard library imports
use std::{
    net::SocketAddr,
};


// Importing crates written by yours, truly <3
use tanks_shared;

// Importing modules
mod networking_plugin;

use networking_plugin::{
    NetworkPlugin,
    websocket_client::WebsocketClient,
    server_connection::ServerConnection,
    events::ConnectionEvent,
};

const SERVER_URL: &str = "wss://127.0.0.1:443";

fn main() {
    let mut bevy_app = App::new();
    bevy_app.add_plugins(DefaultPlugins
        .set(LogPlugin {
            level: bevy::log::Level::INFO,
            filter: "error,tanks_client=info".into(),
        }
    ));
    // NetworkPlugin spawns tokio runtimes (threads) that handle receiving and sending messages.
    bevy_app.add_plugin(NetworkPlugin);


    info!("Setup completed. Running bevy app now (and blocking main thread)");
    bevy_app.run();
}