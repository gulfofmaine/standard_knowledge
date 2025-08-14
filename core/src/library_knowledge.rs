use flate2::read::GzDecoder;
use std::io::Read;

use crate::knowledge::Knowledge;

pub fn load_knowledge() -> Vec<Knowledge> {
    let compressed_data = include_bytes!(concat!(env!("OUT_DIR"), "/knowledge.yaml.gz"));

    // Decompress the data
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut yaml_data = String::new();
    decoder.read_to_string(&mut yaml_data).unwrap();

    // Deserialize from YAML
    let knowledge: Vec<Knowledge> = serde_yaml_ng::from_str(&yaml_data).unwrap();

    knowledge
}
