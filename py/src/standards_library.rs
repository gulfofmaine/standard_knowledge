use std::collections::{BTreeMap, HashMap};

use pyo3::{exceptions::PyKeyError, prelude::*, types::PyDict};

use crate::standard::PyStandard;
use standard_knowledge::qartod::static_qc::StaticQc;
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

    /// Get the standards dictionary
    #[getter]
    fn standards(&self) -> PyResult<HashMap<String, crate::standard::PyStandard>> {
        let mut py_standards = HashMap::new();
        for (key, standard) in &self.0.standards {
            py_standards.insert(key.clone(), crate::standard::PyStandard(standard.clone()));
        }
        Ok(py_standards)
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
            let qc = get_static_qc_field(&know, "qc")?;

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
                qc: Some(qc),
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

    /// Return known IOOS Categories
    fn known_ioos_categories(&self) -> Vec<String> {
        self.0.known_ioos_categories().into_iter().collect()
    }
}

#[derive(Debug)]
enum KnowledgeValues {
    String(String),
    List(Vec<String>),
    Dict(BTreeMap<String, String>),
    QC(BTreeMap<String, StaticQc>),
}

impl<'source> FromPyObject<'source> for KnowledgeValues {
    fn extract_bound(ob: &Bound<'source, PyAny>) -> PyResult<Self> {
        // Try to extract as string first
        if let Ok(s) = ob.extract::<String>() {
            return Ok(KnowledgeValues::String(s));
        }

        // Try to extract as list of strings
        if let Ok(list) = ob.extract::<Vec<String>>() {
            return Ok(KnowledgeValues::List(list));
        }

        // Try to extract as dict of strings
        if let Ok(dict) = ob.extract::<BTreeMap<String, String>>() {
            return Ok(KnowledgeValues::Dict(dict));
        }

        // Try to extract as QC dict (BTreeMap<String, StaticQc>)
        if let Ok(dict) = ob.downcast::<PyDict>() {
            let mut qc_map = BTreeMap::new();

            for (key, value) in dict.iter() {
                let key_str: String = key.extract()?;

                // Extract StaticQc fields from the Python dict
                let value_dict = value.downcast::<PyDict>()?;

                let name: String = value_dict
                    .get_item("name")?
                    .ok_or_else(|| PyKeyError::new_err("StaticQc missing 'name' field"))?
                    .extract()?;

                let summary: String = value_dict
                    .get_item("summary")?
                    .ok_or_else(|| PyKeyError::new_err("StaticQc missing 'summary' field"))?
                    .extract()?;

                let description: String = value_dict
                    .get_item("description")?
                    .ok_or_else(|| PyKeyError::new_err("StaticQc missing 'description' field"))?
                    .extract()?;

                // For tests, we need to handle ConfigStream
                let tests_value = value_dict
                    .get_item("tests")?
                    .ok_or_else(|| PyKeyError::new_err("StaticQc missing 'tests' field"))?;

                // Convert Python tests dict to ConfigStream
                let tests = convert_tests_to_config_stream(&tests_value)?;

                let static_qc = StaticQc {
                    name,
                    summary,
                    description,
                    tests,
                };

                qc_map.insert(key_str, static_qc);
            }

            return Ok(KnowledgeValues::QC(qc_map));
        }

        Err(PyKeyError::new_err(
            "Could not extract KnowledgeValues from Python object",
        ))
    }
}

fn get_static_qc_field(
    knowledge: &HashMap<String, KnowledgeValues>,
    key: &str,
) -> PyResult<BTreeMap<String, StaticQc>> {
    match knowledge.get(key) {
        Some(KnowledgeValues::QC(qc_value)) => Ok(qc_value.clone()),
        Some(_) => Err(PyKeyError::new_err(format!(
            "`{key}` must be a QARTOD static QC field"
        ))),
        None => Ok(BTreeMap::new()),
    }
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

fn convert_tests_to_config_stream(
    tests_value: &Bound<'_, PyAny>,
) -> PyResult<standard_knowledge::qartod::config::ConfigStream> {
    use standard_knowledge::qartod::config::*;

    // Extract the "qartod" field from tests
    let tests_dict = tests_value.downcast::<PyDict>()?;
    let qartod_value = tests_dict
        .get_item("qartod")?
        .ok_or_else(|| PyKeyError::new_err("tests missing 'qartod' field"))?;
    let qartod_dict = qartod_value.downcast::<PyDict>()?;

    let mut config_qartod = ConfigStreamQartod::default();

    // Convert flat_line_test
    if let Some(flat_line_item) = qartod_dict.get_item("flat_line_test")? {
        let flat_line_dict = flat_line_item.downcast::<PyDict>()?;
        let tolerance: f64 = flat_line_dict
            .get_item("tolerance")?
            .ok_or_else(|| PyKeyError::new_err("flat_line_test missing 'tolerance'"))?
            .extract()?;
        let suspect_threshold: isize = flat_line_dict
            .get_item("suspect_threshold")?
            .ok_or_else(|| PyKeyError::new_err("flat_line_test missing 'suspect_threshold'"))?
            .extract()?;
        let fail_threshold: isize = flat_line_dict
            .get_item("fail_threshold")?
            .ok_or_else(|| PyKeyError::new_err("flat_line_test missing 'fail_threshold'"))?
            .extract()?;

        config_qartod.flat_line_test = Some(FlatLine {
            tolerance,
            suspect_threshold,
            fail_threshold,
        });
    }

    // Convert gross_range_test
    if let Some(gross_range_item) = qartod_dict.get_item("gross_range_test")? {
        let gross_range_dict = gross_range_item.downcast::<PyDict>()?;
        let fail_span: Vec<f64> = gross_range_dict
            .get_item("fail_span")?
            .ok_or_else(|| PyKeyError::new_err("gross_range_test missing 'fail_span'"))?
            .extract()?;
        let suspect_span: Vec<f64> = gross_range_dict
            .get_item("suspect_span")?
            .ok_or_else(|| PyKeyError::new_err("gross_range_test missing 'suspect_span'"))?
            .extract()?;

        if fail_span.len() != 2 || suspect_span.len() != 2 {
            return Err(PyKeyError::new_err(
                "fail_span and suspect_span must be arrays of length 2",
            ));
        }

        config_qartod.gross_range_test = Some(GrossRangeTest {
            fail_span: (fail_span[0], fail_span[1]),
            suspect_span: (suspect_span[0], suspect_span[1]),
        });
    }

    // Convert spike_test
    if let Some(spike_item) = qartod_dict.get_item("spike_test")? {
        let spike_dict = spike_item.downcast::<PyDict>()?;
        let suspect_threshold: f64 = spike_dict
            .get_item("suspect_threshold")?
            .ok_or_else(|| PyKeyError::new_err("spike_test missing 'suspect_threshold'"))?
            .extract()?;
        let fail_threshold: f64 = spike_dict
            .get_item("fail_threshold")?
            .ok_or_else(|| PyKeyError::new_err("spike_test missing 'fail_threshold'"))?
            .extract()?;

        config_qartod.spike_test = Some(Spike {
            suspect_threshold,
            fail_threshold,
        });
    }

    // Convert rate_of_change_test
    if let Some(rate_of_change_item) = qartod_dict.get_item("rate_of_change_test")? {
        let rate_of_change_dict = rate_of_change_item.downcast::<PyDict>()?;
        let threshold: f64 = rate_of_change_dict
            .get_item("threshold")?
            .ok_or_else(|| PyKeyError::new_err("rate_of_change_test missing 'threshold'"))?
            .extract()?;

        config_qartod.rate_of_change_test = Some(RateOfChange { threshold });
    }

    Ok(ConfigStream {
        qartod: config_qartod,
    })
}
