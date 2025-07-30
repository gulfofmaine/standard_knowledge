use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CfYaml {
    aliases: HashMap<String, String>,
    standard_names: HashMap<String, CfStandard>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CfStandard {
    description: String,
    unit: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlKnowledge {
    // /// Standard name the knowledge applies to
    // pub name: String,
    /// Human readable name
    pub long_name: Option<String>,

    /// Usual IOOS category for the standard
    pub ioos_category: Option<String>,

    /// Common variable names in a dataset
    pub common_variable_names: Option<Vec<String>>,

    /// Other standards to consider
    pub related_standards: Option<Vec<String>>,

    /// Other units that may be seen
    pub other_units: Option<Vec<String>>,

    /// Community comments on standard usage
    pub comments: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Knowledge {
    // /// Standard name the knowledge applies to
    pub name: String,

    /// Human readable name
    pub long_name: Option<String>,

    /// Usual IOOS category for the standard
    pub ioos_category: Option<String>,

    /// Common variable names in a dataset
    pub common_variable_names: Vec<String>,

    /// Other standards to consider
    pub related_standards: Vec<String>,

    /// Other units that may be seen
    pub other_units: Vec<String>,

    /// Community comments on standard usage
    pub comments: Option<String>,
}

pub fn write_cf_standards_from_yaml() {
    let standard_path = Path::new("standards/_cf_standards.yaml");
    let contents = fs::read_to_string(standard_path).expect("Unable to read standards");

    let cf: CfYaml = serde_yaml_ng::from_str(&contents).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("cf_standards.msgpack");

    let mut buf = Vec::new();
    cf.serialize(&mut Serializer::new(&mut buf)).unwrap();

    fs::write(&dest_path, buf).unwrap()
}

fn find_knowledge() -> Vec<PathBuf> {
    use std::path::Path;

    let mut knowledge_files = Vec::new();

    let path = Path::new("standards");

    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        let file_path = entry.path();

        if let Some(ext) = file_path.extension() {
            if ext == "yaml" && file_path.file_stem().unwrap() != "_cf_standards" {
                knowledge_files.push(file_path);
            }
        }
    }

    knowledge_files
}

fn load_knowledge(path: &PathBuf) -> Knowledge {
    let name = &path.file_stem().unwrap();
    let contents = fs::read_to_string(path).expect("Unable to read knowledge");

    let partial_knowledge: YamlKnowledge = serde_yaml_ng::from_str(&contents).unwrap();
    Knowledge {
        name: name.to_str().unwrap().to_string(),
        long_name: partial_knowledge.long_name,
        ioos_category: partial_knowledge.ioos_category,
        common_variable_names: partial_knowledge.common_variable_names.unwrap_or_default(),
        related_standards: partial_knowledge.related_standards.unwrap_or_default(),
        other_units: partial_knowledge.other_units.unwrap_or_default(),
        comments: partial_knowledge.comments,
    }
}

fn write_knowledge() {
    let knowledge_paths = find_knowledge();
    let mut loaded_knowledge = Vec::new();

    for path in knowledge_paths {
        let knowledge = load_knowledge(&path);
        loaded_knowledge.push(knowledge);
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("knowledge.msgpack");

    let mut buf = Vec::new();
    loaded_knowledge
        .serialize(&mut Serializer::new(&mut buf))
        .unwrap();

    fs::write(&dest_path, buf).unwrap()
}

fn main() {
    write_cf_standards_from_yaml();
    write_knowledge();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=standards/")
}
