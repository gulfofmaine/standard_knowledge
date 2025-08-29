use indicium::simple::SearchIndex;

use crate::standard::Standard;

/// Chainable filter for standards
#[derive(Clone)]
pub struct StandardsFilter {
    pub standards: Vec<Standard>,
}

impl StandardsFilter {
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
    pub fn by_variable_name(&self, variable_name: &str) -> Self {
        let mut standards: Vec<Standard> = self
            .standards
            .iter()
            .filter(|standard| {
                standard
                    .common_variable_names
                    .iter()
                    .any(|name| name == variable_name)
            })
            .cloned()
            .collect();
        standards.sort_by_key(|s| s.name.clone());
        StandardsFilter { standards }
    }

    /// Returns standards by IOOS category
    pub fn by_ioos_category(&self, category: &str) -> Self {
        let mut standards: Vec<Standard> = self
            .standards
            .iter()
            .filter(|standard| {
                standard
                    .ioos_category
                    .as_ref()
                    .is_some_and(|cat| cat.eq_ignore_ascii_case(category))
            })
            .cloned()
            .collect();
        standards.sort_by_key(|s| s.name.clone());
        StandardsFilter { standards }
    }

    /// Returns standards for a given unit
    pub fn by_unit(&self, unit: &str) -> Self {
        let mut standards: Vec<Standard> = self
            .standards
            .iter()
            .filter(|standard| {
                standard.unit == unit || standard.other_units.iter().any(|u| u == unit)
            })
            .cloned()
            .collect();
        standards.sort_by_key(|s| s.name.clone());
        StandardsFilter { standards }
    }

    /// Returns standards that have QARTOD tests
    pub fn has_qartod_tests(&self) -> Self {
        let mut standards: Vec<Standard> = self
            .standards
            .iter()
            .filter(|standard| !standard.qartod.is_empty())
            .cloned()
            .collect();
        standards.sort_by_key(|s| s.name.clone());
        StandardsFilter { standards }
    }

    /// Returns standards that match a search pattern
    pub fn search(&self, search_str: &str) -> Self {
        let mut search_index: SearchIndex<usize> = SearchIndex::default();

        self.standards
            .iter()
            .enumerate()
            .for_each(|(index, element)| search_index.insert(&index, element));
        let results = search_index.search(search_str);

        let mut standards: Vec<Standard> = Vec::new();
        for index in results {
            standards.push(self.standards[*index].clone());
        }

        standards.sort_by_key(|s| s.name.clone());

        StandardsFilter { standards }
    }
}
