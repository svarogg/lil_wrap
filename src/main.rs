mod lil_wrap;

use crate::lil_wrap::wrapped_payload::Type;
use lil_wrap::WrappedPayload;
use prost::bytes::Bytes;
use prost::Message;

const PREFIX: [u8;2] = [0x97, 0xB1];

fn serialize_payload(payload: &WrappedPayload) -> Vec<u8>{
    let mut buf = Vec::new();
    buf.reserve(PREFIX.len() + payload.encoded_len());
    buf.append(PREFIX.to_vec().as_mut());

    payload.encode(&mut buf).unwrap();
    buf
}

fn deserialize_payload(buf: &[u8]) -> Result<WrappedPayload, prost::DecodeError>{
    let (prefix_buf, rest) = buf.split_at(PREFIX.len());
    assert_eq!(prefix_buf, PREFIX);

    let bytes = Bytes::from(rest.to_vec());
    WrappedPayload::decode(&mut bytes.clone())
}

fn main() {
    let wrapped_payload = WrappedPayload{
        version: 0,
        r#type: Type::SmallL2Payload.into(),
        payload: "hello world".to_string().into_bytes(),
    };

    let serialized = serialize_payload(&wrapped_payload);
    println!("Serialized: {:?}", serialized);

    let deserialized = deserialize_payload(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
    let payload: String = String::from_utf8(deserialized.payload).unwrap();
    println!("Deserialized payload: {:?}", payload);
}
