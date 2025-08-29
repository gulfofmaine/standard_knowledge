use std::convert::From;

use indicium::simple::SearchIndex;
use pyo3::exceptions::{PyIndexError, PyKeyError};
use pyo3::prelude::*;

use crate::PyStandard;

use standard_knowledge::Standard;
use standard_knowledge::standards_filter::StandardsFilter;

#[pyclass(name = "StandardsFilter")]
#[derive(Clone)]
pub struct PyStandardsFilter {
    pub standards: Vec<Standard>,
}

impl From<StandardsFilter<'_>> for PyStandardsFilter {
    fn from(filter: StandardsFilter) -> Self {
        PyStandardsFilter {
            standards: filter.standards.into_iter().cloned().collect(),
        }
    }
}

#[pymethods]
impl PyStandardsFilter {
    /// Return standards by common variable name
    fn by_variable_name(&self, py: Python, variable_name: &str) -> PyResult<Py<PyStandardsFilter>> {
        let standards = self
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

        let py_filter = PyStandardsFilter { standards };
        Py::new(py, py_filter)
    }

    /// Return standards by IOOS category
    fn by_ioos_category(&self, py: Python, category: &str) -> PyResult<Py<PyStandardsFilter>> {
        let standards = self
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

        let py_filter = PyStandardsFilter { standards };
        Py::new(py, py_filter)
    }

    /// Return standards for a given unit
    fn by_unit(&self, py: Python, unit: &str) -> PyResult<Py<PyStandardsFilter>> {
        let standards = self
            .standards
            .iter()
            .filter(|standard| {
                standard.unit == unit || standard.other_units.iter().any(|u| u == unit)
            })
            .cloned()
            .collect();

        let py_filter = PyStandardsFilter { standards };
        Py::new(py, py_filter)
    }

    /// Return standards that have QARTOD tests
    fn has_qartod_tests(&self, py: Python) -> PyResult<Py<PyStandardsFilter>> {
        let standards = self
            .standards
            .iter()
            .filter(|standard| !standard.qartod.is_empty())
            .cloned()
            .collect();

        let py_filter = PyStandardsFilter { standards };
        Py::new(py, py_filter)
    }

    /// Return standards that match a search pattern
    fn search(&self, py: Python, search_str: &str) -> PyResult<Py<PyStandardsFilter>> {
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

        let py_filter = PyStandardsFilter { standards };
        Py::new(py, py_filter)
    }

    /// Get a specific standard by name or alias
    fn get(&self, py: Python, standard_name_or_alias: &str) -> PyResult<Py<PyStandard>> {
        for standard in &self.standards {
            if standard.name == standard_name_or_alias
                || standard
                    .aliases
                    .contains(&standard_name_or_alias.to_string())
            {
                let py_standard = PyStandard(standard.clone());
                return Py::new(py, py_standard);
            }
        }
        Err(PyKeyError::new_err("Unknown Standard"))
    }

    /// Return the standards as a list
    fn __iter__(&self, py: Python) -> PyResult<Vec<Py<PyStandard>>> {
        self.standards
            .iter()
            .map(|standard| {
                let py_standard = PyStandard(standard.clone());
                Py::new(py, py_standard)
            })
            .collect()
    }

    /// Return the number of standards in the filter
    fn __len__(&self) -> usize {
        self.standards.len()
    }

    /// Get a standard by index
    fn __getitem__(&self, py: Python, index: usize) -> PyResult<Py<PyStandard>> {
        if index < self.standards.len() {
            let py_standard = PyStandard(self.standards[index].clone());
            Py::new(py, py_standard)
        } else {
            Err(PyIndexError::new_err("Index out of range"))
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "<StandardsFilter: {} standards>",
            self.standards.len()
        ))
    }
}
