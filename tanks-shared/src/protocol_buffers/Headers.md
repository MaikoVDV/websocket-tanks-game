# Headers for ProtoBuf messages
Data is converted into ProtoBuf messages, which are sent in binary form between the server and client.
Before sending the ProtoBuf messages, they are prefixed with a single byte, indicating their header.
This gives the receiving side some idea of how to process the message.

## 0-9: Connection related
1. **Connection established** - Sends the full GameWorld & other state to a client when it connects.
2. **Client connected** - Tells all clients that a new client has connected.
3. **Client disconnected** - Tells all clients that a client has disconnected.

## 10-19: State related
10. **State update** - Sends a state update to all clients. Usually sent on an interval determined by the game loop.
