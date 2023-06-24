use crate::*;

pub struct GameWorld {
    pub clients: HashMap<u32, Client>,
}
impl GameWorld {
    pub fn new() -> GameWorld {
        GameWorld {
            clients: HashMap::new()
        }
    }
    pub fn init(&mut self) {

    }
    pub fn update(&mut self) {

    }
    
    // TODO: MOVE TO ANOTHER FILE
    pub fn add_client(&mut self, client_id: u32) {
        self.clients.insert(
            client_id,
            Client {
                id: client_id,
            },
        );
    }
    pub fn remove_client(&mut self, client_id: u32) {
        self.clients.remove(&client_id);
    }
}