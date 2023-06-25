use crate::*;

pub mod game_world;
pub mod events;
pub mod proto_serialization;

use game_world::GameWorld;
use events::{
    ClientEvents,
    BroadcastEvents,
};

/// The main game loop. Triggers GameWorld updates.
pub async fn run_game_loop(
    broadcast_event_tx: mpsc::UnboundedSender<BroadcastEvents>,
    mut client_input_rx: mpsc::UnboundedReceiver<ClientEvents>,
) {
    println!("Starting game loop!");
    let mut interval = tokio::time::interval(time::Duration::from_millis(1000 / TICKS_PER_SECOND));

    // Initialize the game state
    let mut game_world = GameWorld::new();
    game_world.init();
    // Start the loop
    loop {
        //let start = time::Instant::now();
        tokio::select! {
            game_event = client_input_rx.recv() => {
                if let Some(event) = game_event {
                    match event {
                        ClientEvents::Connected(client_id) => {
                            let mut game_world = GameWorld::new();
                            game_world.add_client(client_id);
                            // let initial_state_message = state_messages::InitialState {
                            //     client_id: conn.id,
                            //     full_state: Some(state_messages::GameStateUpdate {
                            //         // Converting entities & bodies from HashMap to Vec<>
                            //         players: game_world.players.values().cloned().collect(),
                            //         bodies: game_world.bodies.values().cloned().collect(),
                            //     })
                            // };
                            let _ = broadcast_event_tx.send(BroadcastEvents::ClientConnected(client_id));
                        }
                        ClientEvents::Disconnected(client_id) => {
                            game_world.remove_client(client_id);
                            let _ = broadcast_event_tx.send(BroadcastEvents::ClientDisconnected(client_id));
                        }
                        ClientEvents::Input(client_input) => {
                            // game_world.set_input(id, input);
                            //let new_state = proto_all::State::default(); // SOME ACTUAL STATE STILL NEEDS TO BE SAVED AT SOME POINT IN THE FUTURE!!!
                            //let _ = event_sender.send(BroadcastEvents::StateOut(new_state));
                        }
                    }
                }
            }
            _ = interval.tick() => {
                // Update the game state (in our case rapier.rs physics simulation and intersection queries)
                game_world.update();

                // Send the game state to broadcast green thread.
                let _ = broadcast_event_tx.send(BroadcastEvents::StateUpdate);
                // let _ = broadcast_event_sender.send(BroadcastEvents::StateUpdateOut(game_world.get_state_updates()));
                // game_world.game_state_updates.reset();
            }
        }
    }
}