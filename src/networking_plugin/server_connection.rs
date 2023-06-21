use crate::*;

pub struct ServerConnection {
  pub socket_address: SocketAddr,
  listen_task: JoinHandle<()>,
  broadcast_task: JoinHandle<()>,
}
impl ServerConnection {
  // Kill the listening and broadcasting threads.
  pub fn disconnect(self) {
    self.listen_task.abort();
    self.broadcast_task.abort();
  }
}