use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use yaml_rust2::{Yaml, YamlLoader};

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
    let standard_str = include_str!("../standards/_cf_standards.yaml");
    let docs = YamlLoader::load_from_str(standard_str).unwrap();
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

        standard_tuple = format!(
            "{standard_tuple}\n(\"{name}\", \"{unit}\", \"{description}\"),"
        );
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

                const CF_TUPLE: [(&str, &str, &str); {standard_len}] = [{standard_tuple}];
            "
        ),
    )
    .unwrap()
}

fn main() {
    write_cf_standards_from_yaml();
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=standards/")
}
