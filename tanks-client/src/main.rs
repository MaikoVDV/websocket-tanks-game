// Importing crates
use bevy::{
    prelude::*,
    log::LogPlugin,
    tasks::TaskPool,
};
use tokio::{
    runtime,
    runtime::Runtime,
    task::JoinHandle,
    sync::oneshot,
};
// use crossbeam_channel::unbounded;

use url::Url;
// Standard library imports

// Importing crates written by yours, truly <3
// use tanks_shared;

// Importing modules
mod networking_plugin;

use networking_plugin::{
    NetworkPlugin,
    websocket_client::WebsocketClient,
    server_connection::ServerConnection,
    events::ConnectionEvent,
};

const SERVER_IP4: &str = "127.0.0.1";
const SERVER_PORT: &str = "443";

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
