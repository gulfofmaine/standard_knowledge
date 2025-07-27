use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use yaml_rust2::{Yaml, YamlLoader};

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
pub struct YamlSuggestion {
    // /// Standard name the suggestion applies to
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
pub struct Suggestion {
    // /// Standard name the suggestion applies to
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

fn alias_from_yaml(doc: &Yaml) -> HashMap<String, String> {
    let aliases_doc = &doc["aliases"];

    let mut aliases = HashMap::new();

    for (alias_name, standard_name) in aliases_doc.as_hash().unwrap() {
        let alias = alias_name.as_str().unwrap().to_string();
        let standard = standard_name.as_str().unwrap().to_string();
        aliases.insert(alias.clone(), standard.clone());
    }

    aliases
}

pub fn write_cf_standards_from_yaml() {
    let standard_path = Path::new("standards/_cf_standards.yaml");
    let contents = fs::read_to_string(standard_path).expect("Unable to read standards");
    let docs = YamlLoader::load_from_str(contents.as_str()).unwrap();
    let doc = &docs[0];

    let aliases = alias_from_yaml(doc);

    let standard_doc = &doc["standard_names"];

    let mut standard_tuple = String::new();
    let mut standard_len = 0;

    for (name, standard_doc) in standard_doc.as_hash().unwrap() {
        let name = name.as_str().unwrap().to_string();
        let unit: String = standard_doc["unit"].as_str().unwrap_or("").to_string();
        let description = standard_doc["description"]
            .as_str()
            .unwrap_or("")
            .to_string()
            .replace("\"", "\\\"");

        standard_tuple = format!("{standard_tuple}\n(\"{name}\", \"{unit}\", \"{description}\"),");
        standard_len += 1;
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_standards.rs");

    let mut alias_map = String::new();

    for (key, value) in aliases {
        alias_map = format!("{alias_map}\n(\"{key}\", \"{value}\"),")
    }

    fs::write(
        &dest_path,
        format!(
            "
                use std::collections::HashMap;

                pub fn generated_cf_aliases() -> HashMap<&'static str, &'static str> {{
                    HashMap::from([
                        {alias_map}
                    ])
                }}

                static CF_TUPLE: [(&str, &str, &str); {standard_len}] = [{standard_tuple}];
            "
        ),
    )
    .unwrap()
}

pub fn write_serde_cf_standards_from_yaml() {
    let standard_path = Path::new("standards/_cf_standards.yaml");
    let contents = fs::read_to_string(standard_path).expect("Unable to read standards");

    let cf: CfYaml = serde_yaml_ng::from_str(&contents).unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_standards.msgpack");

    let mut buf = Vec::new();
    cf.serialize(&mut Serializer::new(&mut buf)).unwrap();

    fs::write(&dest_path, buf).unwrap()
}

fn find_suggestions() -> Vec<PathBuf> {
    use std::path::Path;

    let mut suggestions_files = Vec::new();

    let path = Path::new("standards");

    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        let file_path = entry.path();

        if let Some(ext) = file_path.extension() {
            if ext == "yaml" && file_path.file_stem().unwrap() != "_cf_standards" {
                suggestions_files.push(file_path);
            }
        }
    }

    suggestions_files
}

fn load_suggestion(path: &PathBuf) -> Suggestion {
    let name = &path.file_stem().unwrap();
    let contents = fs::read_to_string(path).expect("Unable to read suggestion");

    let partial_suggestion: YamlSuggestion = serde_yaml_ng::from_str(&contents).unwrap();
    Suggestion {
        name: name.to_str().unwrap().to_string(),
        long_name: partial_suggestion.long_name,
        ioos_category: partial_suggestion.ioos_category,
        common_variable_names: partial_suggestion.common_variable_names.unwrap_or_default(),
        related_standards: partial_suggestion.related_standards.unwrap_or_default(),
        other_units: partial_suggestion.other_units.unwrap_or_default(),
        comments: partial_suggestion.comments,
    }
}

fn write_suggestions() {
    let suggestion_paths = find_suggestions();
    let mut suggestions = Vec::new();

    for path in suggestion_paths {
        let suggestion = load_suggestion(&path);
        suggestions.push(suggestion);
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("suggestions.msgpack");

    let mut buf = Vec::new();
    suggestions
        .serialize(&mut Serializer::new(&mut buf))
        .unwrap();

    fs::write(&dest_path, buf).unwrap()
}

fn main() {
    write_cf_standards_from_yaml();
    write_serde_cf_standards_from_yaml();
    write_suggestions();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=standards/")
}
