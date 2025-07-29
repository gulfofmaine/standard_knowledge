use crate::{Knowledge, standard::Standard};
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct StandardsLibrary {
    pub standards: HashMap<String, Standard>,
}

impl StandardsLibrary {
    /// Load CF standards from library
    pub fn load_cf_standards(&mut self) {
        use crate::cf::cf_standards;

        self.standards.extend(cf_standards());
    }

    /// Return a standard by name or alias
    pub fn get(&self, standard_name_or_alias: &str) -> Result<Standard, &'static str> {
        if let Some(standard) = self.standards.get(standard_name_or_alias) {
            return Ok(standard.clone());
        }

        for standard in self.standards.values() {
            if standard
                .aliases
                .iter()
                .any(|alias| alias == standard_name_or_alias)
            {
                return Ok(standard.clone());
            }
        }

        Err("Unknown Standard")
    }

    /// Returns standards that match a given column_name
    pub fn by_variable_name(&self, variable_name: &str) -> Vec<Standard> {
        self.standards
            .values()
            .filter(|standard| {
                standard
                    .common_variable_names
                    .iter()
                    .any(|name| name == variable_name)
            })
            .cloned()
            .collect()
    }

    /// Return standards that have a string across multiple fields,
    /// hopefully in a relevant order
    pub fn search(&self, search_str: &str) -> Vec<Standard> {
        let mut standards = Vec::new();

        if let Ok(standard) = self.get(search_str) {
            standards.push(standard);
        }

        let mut by_variable = self.by_variable_name(search_str);
        by_variable.sort_by_key(|s| s.name.clone());

        for standard in by_variable {
            if !standards.contains(&standard) {
                standards.push(standard);
            }
        }

        let mut sorted: Vec<Standard> = self.standards.values().cloned().collect();
        sorted.sort_by_key(|s| s.name.clone());

        // Search for partial matches
        for standard in sorted {
            if !standards.contains(&standard) && standard.matches_pattern(search_str) {
                standards.push(standard.clone());
            }
        }

        standards
    }

    /// Update the loaded standards with knowledge
    pub fn apply_knowledge(&mut self, knowledge: Vec<Knowledge>) {
        for know in knowledge {
            if let Some(standard) = self.standards.get(&know.name) {
                let mut common_variable_names = standard.common_variable_names.clone();
                common_variable_names.append(&mut know.common_variable_names.clone());

                let mut related_standards = standard.related_standards.clone();
                related_standards.append(&mut know.related_standards.clone());

                let mut other_units = standard.other_units.clone();
                other_units.append(&mut know.other_units.clone());

                let new_standard = Standard {
                    long_name: know.long_name,
                    ioos_category: know.ioos_category,
                    common_variable_names,
                    related_standards,
                    other_units: know.other_units,
                    comments: know.comments,
                    ..standard.clone()
                };

                self.standards.insert(know.name, new_standard);
            }
        }
    }

    /// Load community knowledge
    pub fn load_knowledge(&mut self) {
        let knowledge = crate::library_knowledge::load_knowledge();
        self.apply_knowledge(knowledge);
    }

    /// Load test suites
    pub fn load_test_suites(&mut self) {
        use crate::qartod::test_suites;

        let suites = test_suites();
        for (name, suite) in suites {
            if let Some(standard) = self.standards.get(&name) {
                let new_standard = Standard {
                    qartod: suite,
                    ..standard.clone()
                };
                self.standards.insert(name, new_standard);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_standards() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
    }

    #[test]
    fn can_get_standard() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let pressure = library.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }

    #[test]
    fn can_get_standard_by_alias() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let pressure = library.get("air_pressure_at_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }

    #[test]
    fn can_apply_knowledge() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let pressure = library.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
        assert_eq!(pressure.long_name, None);

        let know = Knowledge {
            name: "air_pressure_at_mean_sea_level".to_string(),
            long_name: Some("Air Pressure at Sea Level".to_string()),
            ioos_category: None,
            common_variable_names: Vec::new(),
            related_standards: Vec::new(),
            other_units: Vec::new(),
            comments: None,
        };

        library.apply_knowledge(vec![know]);

        let updated_pressure = library.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(updated_pressure.name, "air_pressure_at_mean_sea_level");
        assert_eq!(
            updated_pressure.long_name.as_ref().unwrap(),
            "Air Pressure at Sea Level"
        );

        assert_ne!(pressure, updated_pressure);
    }

    #[test]
    fn can_find_by_column_name() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let know = Knowledge {
            name: "air_pressure_at_mean_sea_level".to_string(),
            long_name: Some("Air Pressure at Sea Level".to_string()),
            ioos_category: None,
            common_variable_names: vec!["pressure".to_string()],
            related_standards: Vec::new(),
            other_units: Vec::new(),
            comments: None,
        };

        library.apply_knowledge(vec![know]);

        let standards = library.by_variable_name("pressure");
        let pressure = &standards[0];
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }
}
