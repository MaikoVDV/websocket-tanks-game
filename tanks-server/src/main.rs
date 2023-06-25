// Logging and Standard Libary imports
use std::{
    env,
    sync::Arc,
};

use env_logger;
use log::*;
use dotenv::dotenv;

// Tokio!!!
use tokio::{
    net::TcpListener,
    sync::mpsc,
};
use tokio_native_tls::{
    native_tls::{
        TlsAcceptor,
        Identity
    },
};

// Utilities for handling futures, streams and concurrency
use futures_util::stream::StreamExt;
use dashmap::DashMap;

// Importing crates written by yours truly <3
use tanks_shared::game_manager::{
    events::*,
    *
};

// Importing modules
mod networking;

use networking::{
    *,
    listen::listen,
    broadcast::broadcast,
    client_connection::ClientConnection,
};


const SERVER_IP4: &str = "127.0.0.1";
const SERVER_PORT: &str = "443";

#[tokio::main]
async fn main() {
    // Taking variables from the .env file and making them usable as regular environment variables.
    dotenv().ok();
    // Setting up a logger with timestaps
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp_secs()
        .format_module_path(false)
        .format_target(false)
        .format_indent(Some(4))
        .try_init();
    
    let server_address = format!("{}:{}", SERVER_IP4, SERVER_PORT);
    info!("Starting server! Binding listener to {}", server_address);
    
    let tcp_listener = TcpListener::bind(&server_address)
        .await
        .expect("Listening to TCP failed.");

    // Create the TLS acceptor.
    debug!("Creating TLS identity for encrypted connections.");
    let cert_path = match env::var("TLS_CERT_PATH") {
        Ok(value) => value,
        Err(_) => {
            panic!("Couldn't find the directory for the TLS certification. It should be set in the .env file.")
        }
    };
    let certification_password = match env::var("TLS_CERT_PASSWORD") {
        Ok(value) => value,
        Err(_) => {
            panic!("Couldn't find the directory for the TLS certification. It should be set in the .env file.")
        }
    };
    let der: &[u8] = &std::fs::read(&cert_path).expect(format!("Couldn't find the TLS certification file at '{}'.", &cert_path).as_str());
    let cert = Identity::from_pkcs12(der, &certification_password).expect("Couldn't create identity with given certification and password.");

    let tls_acceptor = tokio_native_tls::TlsAcceptor::from(
        TlsAcceptor::builder(cert)
            .build()
            .unwrap()
    );

    // HashMap that stores all of the connections with clients.
    // Put in an Arc, so it can be shared between threads.
    let connections: Arc<DashMap<u32, ClientConnection>> = Arc::new(DashMap::new());
    /*
        Broadcast data to all clients in a seperate async tokio green thread.
        The game loop will use 'broadcast_sender' to send the game state,
        and join&quit events into this function.
    */
    let (broadcast_event_tx, broadcast_event_rx) = mpsc::unbounded_channel::<BroadcastEvents>();
    tokio::spawn(broadcast(broadcast_event_rx, Arc::clone(&connections)));
    
    /*
        Since I will only use one game loop, I'm using an actual std::thread for the game loop.
        This function takes ownership of the 'broadcast_sender' to send events into the 'broadcast' green thread.
    */
    let (client_input_tx, client_input_rx) = mpsc::unbounded_channel::<ClientEvents>();
    tokio::spawn(run_game_loop(broadcast_event_tx, client_input_rx));

    // // A counter to use as client ids.
    let mut id = 0;

    // Accept new clients.
    while let Ok((tcp_stream, peer)) = tcp_listener.accept().await {
        debug!("Received connection request.");
        let tls_acceptor = tls_acceptor.clone();
        let tls_stream = tls_acceptor.accept(tcp_stream).await.unwrap();

        match tokio_tungstenite::accept_async(tls_stream).await {
            Err(e) => info!("Websocket connection error : {}", e),
            Ok(ws_stream) => {
                id += 1;
                info!("New Connection: {} | Set id to: {}", peer, id);
                tokio::spawn(listen(
                    client_input_tx.clone(),
                    ws_stream,
                    Arc::clone(&connections),
                    id
                ));
            }
        }
    }
}
