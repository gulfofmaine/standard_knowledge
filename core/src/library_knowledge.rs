use std::io::Read;
use flate2::read::GzDecoder;

use crate::knowledge::Knowledge;

pub fn load_knowledge() -> Vec<Knowledge> {
    let compressed_data = include_bytes!(concat!(env!("OUT_DIR"), "/knowledge.msgpack.gz"));
    
    // Decompress the data
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut msgpack_data = Vec::new();
    decoder.read_to_end(&mut msgpack_data).unwrap();

    // Deserialize from msgpack
    let knowledge: Vec<Knowledge> = rmp_serde::from_slice(&msgpack_data).unwrap();

    knowledge
}
