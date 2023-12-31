use crate::*;

// Importing modules
pub mod websocket_client;
pub mod server_connection;
pub mod events;

pub mod listen;
pub mod broadcast;

// Importing networking stuff from libraries.
pub use tokio::net::TcpStream;
pub use tokio_tungstenite::{
    WebSocketStream,
    tungstenite::Message,
};
pub use tokio_native_tls::{
    TlsStream,
    {
        native_tls::TlsConnector,
        TlsConnector as TokioTlsConnector
    }
};
pub use futures_util::{
    stream::{
        SplitStream,
        SplitSink,
    },
    StreamExt,
};

/// The NetworkPlugin that gets added to the Bevy app and handles all networking communications.
pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WebsocketClient::new());
        app.add_system(create_new_connection);
    }
}

/// Starts the connection process. Is a Bevy system, so runs every frame and only does something based on some input.
pub fn create_new_connection(
    mut ws_client: ResMut<WebsocketClient>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::R) {
        //let socket_address = format!("ws://127.0.0.1:{}", PORT);
        // let socket_address = SocketAddr::new("127.0.0.1".parse().unwrap(), PORT);

        let server_url: Url = Url::parse(format!("wss://{}:{}", SERVER_IP4, SERVER_PORT).as_str()).unwrap();

        ws_client.connect(server_url);
    }
}
