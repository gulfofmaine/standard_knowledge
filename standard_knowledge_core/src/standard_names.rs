include!(concat!(env!("OUT_DIR"), "/build_standards.rs"));

#[derive(Debug)]
pub struct StandardName {
    name: String,
    unit: String,
    description: String,
    aliases: Vec<String>,
}

pub fn aliases_by_standard_name() -> HashMap<&'static str, Vec<&'static str>> {
    let aliases = cf_aliases();

    let mut standards = HashMap::new();

    for (alias, standard_name) in aliases {
        standards
            .entry(standard_name)
            .or_insert_with(Vec::new)
            .push(alias);
    }

    return standards;
}

pub fn cf_standards() -> HashMap<&'static str, StandardName> {
    let standard_map = load_cf_standard_hashmap();
    let alias_map = aliases_by_standard_name();

    let mut standards = HashMap::new();

    for (name, values) in standard_map {
        let unit = values["unit"].to_string();
        let description = values["description"].to_string();
        let empty_vec = Vec::new();
        let aliases = alias_map.get(name).unwrap_or(&empty_vec);
        let aliases = aliases.iter().map(|alias| alias.to_string()).collect();

        standards.insert(
            name,
            StandardName {
                name: name.to_string(),
                unit,
                description,
                aliases,
            },
        );
    }

    standards
}
