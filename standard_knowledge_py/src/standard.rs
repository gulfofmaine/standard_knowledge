use std::collections::HashMap;

use pyo3::prelude::*;
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

    #[getter]
    fn long_name(&self) -> PyResult<Option<String>> {
        Ok(self.0.long_name.clone())
    }

    #[getter]
    fn unit(&self) -> PyResult<String> {
        Ok(self.0.unit.clone())
    }

    #[getter]
    fn description(&self) -> PyResult<String> {
        Ok(self.0.description.clone())
    }

    #[getter]
    fn aliases(&self) -> PyResult<Vec<String>> {
        Ok(self.0.aliases.clone())
    }

    #[getter]
    fn ioos_category(&self) -> PyResult<Option<String>> {
        Ok(self.0.ioos_category.clone())
    }

    #[getter]
    fn common_variable_names(&self) -> PyResult<Vec<String>> {
        Ok(self.0.common_variable_names.clone())
    }

    #[getter]
    fn related_standards(&self) -> PyResult<Vec<String>> {
        Ok(self.0.related_standards.clone())
    }

    #[getter]
    fn other_units(&self) -> PyResult<Vec<String>> {
        Ok(self.0.other_units.clone())
    }

    #[getter]
    fn comments(&self) -> PyResult<Option<String>> {
        Ok(self.0.comments.clone())
    }

    /// Return a dictionary of Xarray attributes
    fn attrs(&self) -> PyResult<HashMap<&str, &str>> {
        let mut map = HashMap::from([("standard_name", self.0.name.as_str())]);

        if !self.0.unit.is_empty() {
            map.insert("units", self.0.unit.as_str());
        }

        if let Some(long_name) = &self.0.long_name
            && !long_name.is_empty()
        {
            map.insert("long_name", long_name.as_str());
        }

        if let Some(ioos_category) = &self.0.ioos_category
            && !ioos_category.is_empty()
        {
            map.insert("ioos_category", ioos_category.as_str());
        }

        Ok(map)
    }
}
