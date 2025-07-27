use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
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

    /// Display all the fields for a standard
    pub fn display_all(&self) -> String {
        let mut output = format!("{self}");

        if !self.aliases.is_empty() {
            output = format!("{output}\n  Aliases: {}", self.aliases.join(", "))
        }
        if let Some(ioos_category) = &self.ioos_category {
            output = format!("{output}\n  IOOS Category: {ioos_category}")
        }
        if !self.common_variable_names.is_empty() {
            output = format!(
                "{output}\n  Common variables: {}",
                self.common_variable_names.join(", ")
            )
        }
        if !self.related_standards.is_empty() {
            output = format!(
                "{output}\n  Related standards: {}",
                self.related_standards.join(", ")
            )
        }
        if !self.other_units.is_empty() {
            output = format!("{output}\n  Other units: {}", self.other_units.join(", "))
        }
        output = format!("{output}\n\n{}", self.description);
        if let Some(comments) = &self.comments {
            output = format!("{output}\n\nComments: {comments}")
        }

        output
    }

    /// Attributes displayed with Xarray
    pub fn xarray_attrs(&self) -> BTreeMap<&str, &str> {
        let mut map = BTreeMap::from([("standard_name", self.name.as_str())]);

        if !self.unit.is_empty() {
            map.insert("units", self.unit.as_str());
        }

        if let Some(long_name) = &self.long_name
            && !long_name.is_empty()
        {
            map.insert("long_name", long_name.as_str());
        }

        if let Some(ioos_category) = &self.ioos_category
            && !ioos_category.is_empty()
        {
            map.insert("ioos_category", ioos_category.as_str());
        }

        map
    }

    /// Formatted Xarray attributes
    pub fn display_xarray_attrs(&self) -> String {
        let mut output = "{".to_string();
        for (key, value) in self.xarray_attrs() {
            output = format!("{output}\n  \"{key}\": \"{value}\",");
        }
        output = format!("{output}\n}}");
        output
    }

    /// Short format
    pub fn display_short(&self) -> String {
        if let Some(long_name) = &self.long_name {
            return format!("{} - {} - {}", self.name, long_name, self.unit);
        }
        format!("{} - {}", self.name, self.unit)
    }
}

impl fmt::Display for Standard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.display_short())
    }
}

/// A suggestion is a subset of a Standard
#[derive(Clone, Debug, Serialize, Deserialize)]
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
