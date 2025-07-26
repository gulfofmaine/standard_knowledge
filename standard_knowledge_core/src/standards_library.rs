use crate::{Suggestion, standard::Standard};
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

    // Load and apply library suggestions

    /// Return a standard by name or alias
    pub fn get(&self, standard_name_or_alias: &str) -> Result<Standard, &'static str> {
        if let Some(standard) = self.standards.get(standard_name_or_alias) {
            return Ok(standard.clone());
        }

        for standard in self.standards.values() {
            if standard
                .aliases
                .contains(&standard_name_or_alias.to_string())
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
                    .contains(&variable_name.to_string())
            })
            .cloned()
            .collect()
    }

    /// Update the loaded standards with suggestions
    pub fn apply_suggestions(&mut self, suggestions: Vec<Suggestion>) {
        for suggestion in suggestions {
            if let Some(standard) = self.standards.get(&suggestion.name) {
                let mut common_variable_names = standard.common_variable_names.clone();
                common_variable_names.append(&mut suggestion.common_variable_names.clone());

                let mut related_standards = standard.related_standards.clone();
                related_standards.append(&mut suggestion.related_standards.clone());

                let mut other_units = standard.other_units.clone();
                other_units.append(&mut suggestion.other_units.clone());

                let new_standard = Standard {
                    long_name: suggestion.long_name,
                    ioos_category: suggestion.ioos_category,
                    common_variable_names,
                    related_standards,
                    other_units: suggestion.other_units,
                    comments: suggestion.comments,
                    ..standard.clone()
                };

                self.standards.insert(suggestion.name, new_standard);
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
    fn can_apply_suggestions() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let pressure = library.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
        assert_eq!(pressure.long_name, None);

        let suggestion = Suggestion {
            name: "air_pressure_at_mean_sea_level".to_string(),
            long_name: Some("Air Pressure at Sea Level".to_string()),
            ioos_category: None,
            common_variable_names: Vec::new(),
            related_standards: Vec::new(),
            other_units: Vec::new(),
            comments: None,
        };

        library.apply_suggestions(vec![suggestion]);

        let updated_pressure = library.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(updated_pressure.name, "air_pressure_at_mean_sea_level");
        assert_eq!(
            updated_pressure.long_name.clone().unwrap(),
            "Air Pressure at Sea Level"
        );

        assert_ne!(pressure, updated_pressure);
    }

    #[test]
    fn can_find_by_column_name() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let suggestion = Suggestion {
            name: "air_pressure_at_mean_sea_level".to_string(),
            long_name: Some("Air Pressure at Sea Level".to_string()),
            ioos_category: None,
            common_variable_names: vec!["pressure".to_string()],
            related_standards: Vec::new(),
            other_units: Vec::new(),
            comments: None,
        };

        library.apply_suggestions(vec![suggestion]);

        let standards = library.by_variable_name("pressure");
        let pressure = &standards[0];
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }
}
