use std::collections::{BTreeMap, HashMap};

use pyo3::{exceptions::PyKeyError, prelude::*};

use crate::standard::PyStandard;
use standard_knowledge::{Knowledge, StandardsLibrary};

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

    /// Apply knowledge to loaded standards
    fn apply_knowledge(
        &mut self,
        knowledge: Vec<HashMap<String, KnowledgeValues>>,
    ) -> PyResult<()> {
        let mut cleaned_knowledge = Vec::new();

        for know in knowledge {
            let name;
            if let Some(value) = know.get("name") {
                if let KnowledgeValues::String(str_value) = value {
                    name = str_value.clone();
                } else {
                    return Err(PyKeyError::new_err("`name` is not a string in knowledge"));
                }
            } else {
                return Err(PyKeyError::new_err(
                    "The knowledge needs a name of the standard to be applied to",
                ));
            }

            let long_name = get_string_field(&know, "long_name")?;
            let ioos_category = get_string_field(&know, "ioos_category")?;
            let comments = get_string_field(&know, "comments")?;

            let common_variable_names = get_list_field(&know, "common_variable_names")?;
            let related_standards = get_list_field(&know, "related_standards")?;
            let sibling_standards = get_list_field(&know, "sibling_standards")?;
            let extra_attrs = get_dict_field(&know, "extra_attrs")?;
            let other_units = get_list_field(&know, "other_units")?;

            let cleaned = Knowledge {
                name,
                long_name,
                ioos_category,
                common_variable_names,
                related_standards,
                sibling_standards,
                extra_attrs,
                other_units,
                comments,
            };
            cleaned_knowledge.push(cleaned);
        }

        self.0.apply_knowledge(cleaned_knowledge);

        Ok(())
    }

    /// Load community knowledge baked into the library
    fn load_knowledge(&mut self) {
        self.0.load_knowledge();
    }

    /// Return a standards filter for chaining operations
    fn filter(&self, py: Python) -> PyResult<Py<crate::PyStandardsFilter>> {
        let filter = self.0.filter();
        let py_filter = crate::PyStandardsFilter::from(filter);
        Py::new(py, py_filter)
    }
}

#[derive(FromPyObject, Debug)]
enum KnowledgeValues {
    String(String),
    List(Vec<String>),
    Dict(BTreeMap<String, String>),
}

fn get_string_field(
    knowledge: &HashMap<String, KnowledgeValues>,
    key: &str,
) -> PyResult<Option<String>> {
    match knowledge.get(key) {
        Some(KnowledgeValues::String(str_value)) => Ok(Some(str_value.clone())),
        Some(_) => Err(PyKeyError::new_err(format!(
            "`{key}` must be a string field"
        ))),
        None => Ok(None),
    }
}

fn get_list_field(
    knowledge: &HashMap<String, KnowledgeValues>,
    key: &str,
) -> PyResult<Vec<String>> {
    match knowledge.get(key) {
        Some(KnowledgeValues::List(list_value)) => Ok(list_value.clone()),
        Some(_) => Err(PyKeyError::new_err(format!(
            "`{key}` must be a list of strings"
        ))),
        None => Ok(Vec::new()),
    }
}

fn get_dict_field(
    knowledge: &HashMap<String, KnowledgeValues>,
    key: &str,
) -> PyResult<BTreeMap<String, String>> {
    match knowledge.get(key) {
        Some(KnowledgeValues::Dict(dict_value)) => Ok(dict_value.clone()),
        Some(_) => Err(PyKeyError::new_err(format!(
            "`{key}` must be a dictionary of strings"
        ))),
        None => Ok(BTreeMap::new()),
    }
}
