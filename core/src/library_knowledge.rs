use crate::standard::Knowledge;

pub fn load_knowledge() -> Vec<Knowledge> {
    let msg = include_bytes!(concat!(env!("OUT_DIR"), "/knowledge.msgpack"));

    let knowledge: Vec<Knowledge> = rmp_serde::from_slice(msg).unwrap();

    knowledge
}
