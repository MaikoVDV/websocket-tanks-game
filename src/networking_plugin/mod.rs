use crate::*;

// Importing modules
pub mod websocket_client;
pub mod server_connection;
pub mod events;

// The NetworkPlugin that gets added to the Bevy app and handles all networking communications.
pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(WebsocketClient::new());
    app.add_system(create_new_connection);
  }
}

pub fn create_new_connection(
  mut ws_client: ResMut<WebsocketClient>,
  keys: Res<Input<KeyCode>>,
) {
  if keys.just_pressed(KeyCode::R) {
      //let socket_address = format!("ws://127.0.0.1:{}", PORT);
      //let socket_address = SocketAddr::new("127.0.0.1".parse().unwrap(), PORT);
      ws_client.connect(SERVER_URL);
  }
}