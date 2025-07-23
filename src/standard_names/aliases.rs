use std::collections::HashMap;

#[derive(Debug)]
pub struct AliasContainer {
    pub aliases: HashMap<&'static str, &'static str>,
    pub standards: HashMap<&'static str, Vec<&'static str>>,
}

pub fn load_aliases() -> AliasContainer {
    let raw_aliases = include_str!("raw_aliases.tsv");
    let mut aliases = HashMap::new();

    let mut standards = HashMap::new();

    for line in raw_aliases.lines() {
        let mut parts = line.split('\t');
        let alias = parts.next().unwrap();
        let standard_name = parts.next().unwrap();
        aliases.insert(alias, standard_name);

        standards
            .entry(standard_name)
            .or_insert_with(Vec::new)
            .push(alias);
    }
    AliasContainer { aliases, standards }
}
