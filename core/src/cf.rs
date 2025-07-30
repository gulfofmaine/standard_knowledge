use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::standard::Standard;

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

fn load_cf_msgpack() -> CfYaml {
    let msg = include_bytes!(concat!(env!("OUT_DIR"), "/cf_standards.msgpack"));
    rmp_serde::from_slice(msg).unwrap()
}

/// Returns a HashMap of standard names: vector of aliases
fn aliases_by_standard_name(cf_yaml: &CfYaml) -> HashMap<String, Vec<String>> {
    let aliases = &cf_yaml.aliases;

    let mut standards = HashMap::new();

    for (alias, standard_name) in aliases {
        standards
            .entry(standard_name.clone())
            .or_insert_with(Vec::new)
            .push(alias.clone());
    }

    standards
}

/// Returns a HashMap of standard names to Standard
pub fn cf_standards() -> HashMap<String, Standard> {
    let cf_yaml = load_cf_msgpack();
    let alias_map = aliases_by_standard_name(&cf_yaml);

    let mut standards = HashMap::new();

    for (name, cf_standard) in &cf_yaml.standard_names {
        let empty_vec = Vec::new();
        let aliases = alias_map.get(name).unwrap_or(&empty_vec);

        standards.insert(
            name.to_string(),
            Standard {
                name: name.to_string(),
                unit: cf_standard.unit.clone(),
                description: cf_standard.description.clone(),
                aliases: aliases.to_vec(),
                ..Standard::default()
            },
        );
    }

    standards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_cf_standards() {
        let standards = cf_standards();
        let pressure = standards["air_pressure_at_mean_sea_level"].clone();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");

        println!("Name is correct");

        assert!(
            pressure
                .aliases
                .contains(&"air_pressure_at_sea_level".to_string()),
            "The standard `air_pressure_at_mean_sea_level` should contain the alias `air_pressure_at_sea_level`"
        )
    }
}
