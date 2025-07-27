use crate::standard::Suggestion;

pub fn load_suggestions() -> Vec<Suggestion> {
    let msg = include_bytes!(concat!(env!("OUT_DIR"), "/suggestions.msgpack"));

    let suggestions: Vec<Suggestion> = rmp_serde::from_slice(msg).unwrap();

    suggestions
}
