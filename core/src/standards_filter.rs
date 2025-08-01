use crate::standard::Standard;

/// Chainable filter for standards
#[derive(Clone)]
pub struct StandardsFilter<'a> {
    pub standards: Vec<&'a Standard>,
}

impl<'a> StandardsFilter<'a> {
    /// Return a standard by name or alias
    pub fn get(&self, standard_name_or_alias: &str) -> Result<&Standard, &'static str> {
        for standard in &self.standards {
            if standard.name == standard_name_or_alias
                || standard
                    .aliases
                    .contains(&standard_name_or_alias.to_string())
            {
                return Ok(standard);
            }
        }
        Err("Unknown Standard")
    }

    /// Returns standards by common variable name
    pub fn by_variable_name(self, variable_name: &str) -> Self {
        let mut standards: Vec<&Standard> = self
            .standards
            .into_iter()
            .filter(|standard| {
                standard
                    .common_variable_names
                    .iter()
                    .any(|name| name == variable_name)
            })
            .collect();
        standards.sort_by_key(|s| s.name.clone());
        StandardsFilter { standards }
    }

    /// Returns standards by IOOS category
    pub fn by_ioos_category(self, category: &str) -> Self {
        let mut standards: Vec<&Standard> = self
            .standards
            .into_iter()
            .filter(|standard| {
                standard
                    .ioos_category
                    .as_ref()
                    .is_some_and(|cat| cat.eq_ignore_ascii_case(category))
            })
            .collect();
        standards.sort_by_key(|s| s.name.clone());
        StandardsFilter { standards }
    }

    /// Returns standards for a given unit
    pub fn by_unit(self, unit: &str) -> Self {
        let mut standards: Vec<&'a Standard> = self
            .standards
            .into_iter()
            .filter(|standard| {
                standard.unit == unit || standard.other_units.iter().any(|u| u == unit)
            })
            .collect();
        standards.sort_by_key(|s| s.name.clone());
        StandardsFilter { standards }
    }

    /// Returns standards that have QARTOD tests
    pub fn has_qartod_tests(self) -> Self {
        let mut standards: Vec<&Standard> = self
            .standards
            .into_iter()
            .filter(|standard| !standard.qartod.is_empty())
            .collect();
        standards.sort_by_key(|s| s.name.clone());
        StandardsFilter { standards }
    }

    /// Returns standards that match a search pattern
    pub fn search(self, search_str: &str) -> Self {
        let mut standards = Vec::new();

        // First, try to find exact match by name or alias
        for standard in &self.standards {
            if standard.name == search_str || standard.aliases.contains(&search_str.to_string()) {
                standards.push(*standard);
                break;
            }
        }

        // Create a new filter from current standards to search by variable name
        let by_variable_filter = StandardsFilter {
            standards: self.standards.clone(),
        }
        .by_variable_name(search_str);
        let mut by_variable = by_variable_filter.standards;
        by_variable.sort_by_key(|s| s.name.clone());

        for standard in by_variable {
            if !standards.contains(&standard) {
                standards.push(standard);
            }
        }

        let mut sorted = self.standards.clone();
        sorted.sort_by_key(|s| s.name.clone());

        // Search for partial matches
        for standard in sorted {
            if !standards.contains(&standard) && standard.matches_pattern(search_str) {
                standards.push(standard);
            }
        }

        StandardsFilter { standards }
    }
}
