use std::collections::HashMap;

use pyo3::{
    exceptions::{PyKeyError, PyValueError},
    prelude::*,
    types::{PyTuple, PyType},
};

use crate::standard::PyStandard;
use standard_knowledge::{Standard, StandardsLibrary, Suggestion};

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

    fn apply_suggestions(
        &mut self,
        py: Python,
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

            let mut long_name = None;
            if let Some(value) = suggestion.get("long_name") {
                if let SuggestionValues::String(str_value) = value {
                    long_name = Some(str_value.to_string());
                } else {
                    return Err(PyKeyError::new_err("`long_name` can only be a string"));
                }
            }

            let mut ioos_category = None;
            if let Some(value) = suggestion.get("ioos_category") {
                if let SuggestionValues::String(str_value) = value {
                    ioos_category = Some(str_value.to_string());
                } else {
                    return Err(PyKeyError::new_err("`ioos_category` must be a string"));
                }
            }

            let cleaned = Suggestion {
                name,
                long_name,
                ioos_category,
                common_variable_names: Vec::new(),
                related_standards: Vec::new(),
                other_units: Vec::new(),
                comments: None,
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