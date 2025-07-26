#[derive(Clone, Debug, Default, PartialEq)]
pub struct Standard {
    pub name: String,

    /// Human readable name
    pub long_name: Option<String>,
    pub unit: String,
    pub description: String,
    pub aliases: Vec<String>,

    /// Usual IOOS category for the standard
    pub ioos_category: Option<String>,

    /// Common variable names in a dataset
    pub common_variable_names: Vec<String>,

    /// Other standards to consider
    pub related_standards: Vec<String>,

    /// Other units that may be seen
    pub other_units: Vec<String>,

    /// Community comments on standard usage
    pub comments: Option<String>,
}

impl Standard {
    /// Do any of the fields in the standard match a search pattern
    pub fn matches_pattern(&self, search_str: &str) -> bool {
        let search_str = search_str.to_lowercase();
        let search_str = search_str.as_str();
        self.name.to_lowercase().contains(search_str)
            || self
                .long_name
                .as_ref()
                .is_some_and(|name| name.to_lowercase().contains(search_str))
            || self.unit.to_lowercase().contains(search_str)
            || self.description.to_lowercase().contains(search_str)
            || self
                .aliases
                .iter()
                .any(|alias| alias.to_lowercase().contains(search_str))
            || self
                .ioos_category
                .as_ref()
                .is_some_and(|category| category.to_lowercase().contains(search_str))
            || self
                .common_variable_names
                .iter()
                .any(|name| name.to_lowercase().contains(search_str))
            || self
                .related_standards
                .iter()
                .any(|name| name.to_lowercase().contains(search_str))
            || self
                .other_units
                .iter()
                .any(|unit| unit.to_lowercase().contains(search_str))
            || self
                .comments
                .as_ref()
                .is_some_and(|comment| comment.to_lowercase().contains(search_str))
    }
}

/// A suggestion is a subset of a Standard
#[derive(Clone, Debug)]
pub struct Suggestion {
    /// Standard name the suggestion applies to
    pub name: String,

    /// Human readable name
    pub long_name: Option<String>,

    /// Usual IOOS category for the standard
    pub ioos_category: Option<String>,

    /// Common variable names in a dataset
    pub common_variable_names: Vec<String>,

    /// Other standards to consider
    pub related_standards: Vec<String>,

    /// Other units that may be seen
    pub other_units: Vec<String>,

    /// Community comments on standard usage
    pub comments: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_match_standard_by_long_name() {
        let standard = Standard {
            name: "air_pressure_at_mean_sea_level".to_string(),
            long_name: None,
            unit: "Pa".to_string(),
            description: "A quick note".to_string(),
            aliases: Vec::new(),
            ioos_category: Some("Meteorology".to_string()),
            common_variable_names: Vec::new(),
            related_standards: Vec::new(),
            other_units: Vec::new(),
            comments: None,
        };

        assert!(
            standard.matches_pattern("Met"),
            "Should be able to find met within the standard",
        );

        assert!(
            !standard.matches_pattern("Nothing"),
            "Shouldn't match something random"
        );
    }
}
