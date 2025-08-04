use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::config::ConfigStream;
use super::types::{QartodTestTypes, TestSuite, TestSuiteInfo};

include!("./static_qc_include.rs");

#[derive(Debug, Clone)]
pub struct StaticQcTestSuite {
    pub slug: String,
    pub qc: StaticQc,
}

impl TestSuite for StaticQcTestSuite {
    fn info(&self) -> TestSuiteInfo {
        let test_types = QartodTestTypes::tests_in_config(&self.qc.tests);
        TestSuiteInfo {
            slug: self.slug.clone(),
            name: self.qc.name.clone(),
            summary: self.qc.summary.clone(),
            description: self.qc.description.clone(),
            arguments: HashMap::new(), // Static QC does not have arguments
            test_types,
        }
    }

    fn scaffold(
        &self,
        _arguments: std::collections::HashMap<String, super::types::ArgumentValue>,
    ) -> Result<ConfigStream, String> {
        Ok(self.qc.tests.clone())
    }
}
