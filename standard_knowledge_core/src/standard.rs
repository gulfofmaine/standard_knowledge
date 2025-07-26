use crate::ioos_categories::IOOSCategory;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Standard {
    pub name: String,

    /// Human readable name
    pub long_name: Option<String>,
    pub unit: String,
    pub description: String,
    pub aliases: Vec<String>,

    /// Usual IOOS category for the standard
    pub ioos_category: Option<IOOSCategory>,

    /// Common variable names in a dataset
    pub common_variable_names: Vec<String>,

    /// Other standards to consider
    pub related_standards: Vec<String>,

    /// Other units that may be seen
    pub other_units: Vec<String>,

    /// Community comments on standard usage
    pub comments: Option<String>,
}

/// A suggestion is a subset of a Standard
#[derive(Clone, Debug)]
pub struct Suggestion {
    /// Standard name the suggestion applies to
    pub name: String,

    /// Human readable name
    pub long_name: Option<String>,

    /// Usual IOOS category for the standard
    pub ioos_category: Option<IOOSCategory>,

    /// Common variable names in a dataset
    pub common_variable_names: Vec<String>,

    /// Other standards to consider
    pub related_standards: Vec<String>,

    /// Other units that may be seen
    pub other_units: Vec<String>,

    /// Community comments on standard usage
    pub comments: Option<String>,
}
