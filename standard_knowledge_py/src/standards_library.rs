use std::collections::HashMap;

use pyo3::{exceptions::PyKeyError, prelude::*};

use crate::standard::PyStandard;
use standard_knowledge::{StandardsLibrary, Suggestion};

#[pyclass(name = "StandardsLibrary")]
#[derive(Clone)]
pub struct PyStandardsLibrary(pub StandardsLibrary);

/// A collection of CF compatible standards with methods for searching through them
#[pymethods]
impl PyStandardsLibrary {
    #[new]
    fn new() -> Self {
        Self(StandardsLibrary {
            standards: HashMap::new(),
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "<StandardsLibrary: {} standards>",
            self.0.standards.len()
        ))
    }

    /// Load CF standards into library
    fn load_cf_standards(&mut self) {
        self.0.load_cf_standards();
    }

    /// Get a standard by standard name or aliases
    fn get(&self, py: Python, name_or_alias: &str) -> PyResult<Py<PyStandard>> {
        match self.0.get(name_or_alias) {
            Ok(standard) => {
                let py_standard = PyStandard(standard);
                Py::new(py, py_standard)
            }
            Err(e) => Err(PyKeyError::new_err(e.to_string())),
        }
    }

    /// Return standards that match a given variable name
    fn by_variable_name(&self, variable_name: &str) -> PyResult<Vec<PyStandard>> {
        let standards = self.0.by_variable_name(variable_name);

        Ok(standards
            .iter()
            .map(|standard| PyStandard(standard.clone()))
            .collect())
    }

    /// Return standards that have a string across multiple fields,
    /// hopefully in a relevant order
    fn search(&self, search_str: &str) -> PyResult<Vec<PyStandard>> {
        let standards = self.0.search(search_str);

        Ok(standards
            .iter()
            .map(|standard| PyStandard(standard.clone()))
            .collect())
    }

    /// Apply suggestions to loaded standards
    fn apply_suggestions(
        &mut self,
        suggestions: Vec<HashMap<String, SuggestionValues>>,
    ) -> PyResult<()> {
        let mut cleaned_suggestions = Vec::new();

        for suggestion in suggestions {
            let name;
            if let Some(value) = suggestion.get("name") {
                if let SuggestionValues::String(str_value) = value {
                    name = str_value.to_string();
                } else {
                    return Err(PyKeyError::new_err("`name` is not a string in suggestion"));
                }
            } else {
                return Err(PyKeyError::new_err(
                    "The suggestion needs a name of the standard to be applied to",
                ));
            }

            let long_name = get_string_field(&suggestion, "long_name")?;
            let ioos_category = get_string_field(&suggestion, "ioos_category")?;
            let comments = get_string_field(&suggestion, "comments")?;

            let common_variable_names = get_list_field(&suggestion, "common_variable_names")?;
            let related_standards = get_list_field(&suggestion, "related_standards")?;
            let other_units = get_list_field(&suggestion, "other_units")?;

            let cleaned = Suggestion {
                name,
                long_name,
                ioos_category,
                common_variable_names,
                related_standards,
                other_units,
                comments,
            };
            cleaned_suggestions.push(cleaned);
        }

        self.0.apply_suggestions(cleaned_suggestions);

        Ok(())
    }
}

#[derive(FromPyObject, Debug)]
enum SuggestionValues {
    String(String),
    List(Vec<String>),
}

fn get_string_field(
    suggestion: &HashMap<String, SuggestionValues>,
    key: &str,
) -> PyResult<Option<String>> {
    match suggestion.get(key) {
        Some(SuggestionValues::String(str_value)) => Ok(Some(str_value.to_string())),
        Some(_) => Err(PyKeyError::new_err(format!(
            "`{key}` must be a string field"
        ))),
        None => Ok(None),
    }
}

fn get_list_field(
    suggestion: &HashMap<String, SuggestionValues>,
    key: &str,
) -> PyResult<Vec<String>> {
    match suggestion.get(key) {
        Some(SuggestionValues::List(list_value)) => Ok(list_value.clone()),
        Some(_) => Err(PyKeyError::new_err(format!(
            "`{key}` must be a list of strings"
        ))),
        None => Ok(Vec::new()),
    }
}
