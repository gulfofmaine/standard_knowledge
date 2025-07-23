pub mod aliases;
pub mod ioos_categories;

use std::collections::HashMap;

use ioos_categories::IOOSCategory;

#[derive(Debug)]
pub struct StandardName {
    name: String,
    unit: String,
    description: String,
    aliases: Vec<String>,
    ioos_category: Option<IOOSCategory>,
}

// trait Attributes {
//     fn attrs(&self) -> HashMap<&'static str, &'static str>;
// }

impl StandardName {
    fn attrs(&self) -> HashMap<&str, &String> {
        let mut attrs = HashMap::new();
        attrs.insert("name", &self.name);
        attrs.insert("unit", &self.unit);
        attrs.insert("description", &self.description);
        // attrs.insert("aliases", &self.aliases);
        // if let Some(ioos_category) = &self.ioos_category {
        //     attrs.insert("ioos_category", &ioos_category.as_str());
        // }
        attrs
    }
}

pub fn load_standards() -> HashMap<String, StandardName> {
    use aliases::load_aliases;

    let aliases = load_aliases();

    let raw_standards = include_str!("raw_standard_names.tsv");
    let mut standards = HashMap::new();

    for line in raw_standards.lines() {
        let mut parts = line.split('\t');
        let name = parts.next();
        let unit = parts.next();
        let description = parts.next();

        let empty_vec = Vec::new();
        let aliases_for_standard = aliases.standards.get(name.unwrap()).unwrap_or(&empty_vec);

        let standard = StandardName {
            name: name.unwrap().to_string(),
            unit: unit.unwrap().to_string(),
            description: description.unwrap().to_string(),
            aliases: aliases_for_standard
                .iter()
                .map(|alias| alias.to_string())
                .collect(),
            ioos_category: None,
        };

        standards.insert(name.unwrap().to_string(), standard);
    }
    standards
}
