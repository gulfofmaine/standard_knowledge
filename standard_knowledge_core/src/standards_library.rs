use crate::standard::Standard;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct StandardsLibrary {
    pub standards: HashMap<&'static str, Standard>,
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

    // /// Returns standards that may match a column_name
    // pub fn for_column(&self, column_name: &str) -> Result<Vec<Standard>, &'static str> {

    //     Err("No standards found")
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_standards() {
        let mut standards = StandardsLibrary::default();
        standards.load_cf_standards();
    }

    #[test]
    fn can_get_standard() {
        let mut standards = StandardsLibrary::default();
        standards.load_cf_standards();
        let pressure = standards.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }

    #[test]
    fn can_get_standard_by_alias() {
        let mut standards = StandardsLibrary::default();
        standards.load_cf_standards();
        let pressure = standards.get("air_pressure_at_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }
}
