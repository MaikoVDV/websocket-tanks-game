use quick_protobuf::Writer;
pub use quick_protobuf;

/// Serialize a struct into a ProtoBuf message to be put in a Message::Binary that will be sent over the websocket.
pub fn proto_serialize<T: quick_protobuf::MessageWrite>(data: T, message_header: u8) -> Vec<u8> {
    let mut out = Vec::new();
    let mut writer = Writer::new(&mut out);

    writer.write_u8(message_header).unwrap();
    //quick_protobuf::serialize_into_slice(&data, &mut out).expect(&format!("Failed to serialize message with header {}", message_header));
    writer
        .write_message(&data)
        .expect("Cannot serialize message.");

    return out;
}

// pub fn proto_deserialize<'a, M: quick_protobuf::MessageRead<'a>>(data: &'a [u8]) -> Result<M> {

// }