use std::collections::HashMap;

use pyo3::{
    exceptions::{PyKeyError, PyValueError},
    prelude::*,
    types::{PyTuple, PyType},
};
use standard_knowledge::{Standard, Suggestion};

#[pyclass(name = "Standard")]
#[derive(Clone)]
pub struct PyStandard(pub Standard);

/// A CF compatible standard
#[pymethods]
impl PyStandard {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<Standard: {}>", self.0.name))
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.0.name.clone())
    }

    #[getter]
    fn long_name(&self) -> PyResult<Option<String>> {
        Ok(self.0.long_name.clone())
    }

    /// Return a dictionary of Xarray attributes
    fn attrs(&self) -> PyResult<HashMap<&str, &str>> {
        let mut map = HashMap::from([("standard_name", self.0.name.as_str())]);

        if self.0.unit != "" {
            map.insert("units", self.0.unit.as_str());
        }

        if let Some(long_name) = &self.0.long_name
            && long_name != ""
        {
            map.insert("long_name", long_name.as_str());
        }

        if let Some(ioos_category) = &self.0.ioos_category
            && ioos_category != ""
        {
            map.insert("ioos_category", ioos_category.as_str());
        }

        Ok(map)
    }
}

#[pyclass(name = "Suggestion")]
#[derive(Clone)]
pub struct PySuggestion(Suggestion);
