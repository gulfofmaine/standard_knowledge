use std::collections::HashMap;

use pyo3::{
    exceptions::{PyKeyError, PyValueError},
    prelude::*,
    types::{PyTuple, PyType},
};
use standard_knowledge::Standard;

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

    /// Return a dictionary of Xarray attributes
    fn attrs(&self) -> PyResult<HashMap<&str, &str>> {
        let mut map = HashMap::from([("standard_name", self.0.name.as_str())]);

        if self.0.unit != "" {
            map.insert("units", self.0.unit.as_str());
        }

        Ok(map)
    }
}
