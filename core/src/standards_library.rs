use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::qartod::StaticQcTestSuite;
use crate::standards_filter::StandardsFilter;
use crate::{Knowledge, standard::Standard};

#[derive(Debug, Default, Clone)]
pub struct StandardsLibrary {
    pub standards: HashMap<String, Standard>,
}

impl StandardsLibrary {
    /// Load CF standards from library
    pub fn load_cf_standards(&mut self) {
        use crate::cf::cf_standards;

        self.standards.extend(cf_standards());
    }

    pub fn filter(&self) -> StandardsFilter {
        StandardsFilter {
            standards: self.standards.values().cloned().collect(),
        }
    }

    /// Return a standard by name or alias
    pub fn get(&self, standard_name_or_alias: &str) -> Result<Standard, &'static str> {
        let filter = self.filter();
        let standard = filter.get(standard_name_or_alias)?;
        Ok(standard.clone())
    }

    /// Update the loaded standards with knowledge
    pub fn apply_knowledge(&mut self, knowledge: Vec<Knowledge>) {
        for know in knowledge {
            if let Some(standard) = self.standards.get(&know.name) {
                let mut common_variable_names = standard.common_variable_names.clone();
                common_variable_names.append(&mut know.common_variable_names.clone());

                let mut related_standards = standard.related_standards.clone();
                related_standards.append(&mut know.related_standards.clone());

                let mut sibling_standards = standard.sibling_standards.clone();
                sibling_standards.append(&mut know.sibling_standards.clone());

                let mut extra_attrs = standard.extra_attrs.clone();
                for (key, value) in know.extra_attrs {
                    extra_attrs.insert(key, value);
                }

                let mut other_units = standard.other_units.clone();
                other_units.append(&mut know.other_units.clone());

                let mut qartod = standard.qartod.clone();

                if let Some(qc) = know.qc {
                    for (slug, qc) in qc {
                        qartod.push(Box::new(StaticQcTestSuite { slug, qc }));
                    }
                }

                let new_standard = Standard {
                    long_name: know.long_name,
                    ioos_category: know.ioos_category,
                    common_variable_names,
                    related_standards,
                    sibling_standards,
                    extra_attrs,
                    other_units: know.other_units,
                    comments: know.comments,
                    qartod,
                    ..standard.clone()
                };

                self.standards.insert(know.name, new_standard);
            }
        }
    }

    /// Load community knowledge
    pub fn load_knowledge(&mut self) {
        let knowledge = crate::library_knowledge::load_knowledge();
        self.apply_knowledge(knowledge);
    }

    /// Load knowledge from a file path (single file or directory)
    pub fn load_knowledge_from_path<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        
        if path.is_file() {
            // Load single file
            let knowledge = self.load_single_knowledge_file(path)?;
            self.apply_knowledge(vec![knowledge]);
        } else if path.is_dir() {
            // Load all YAML files from directory
            let knowledge = self.load_knowledge_from_directory(path)?;
            self.apply_knowledge(knowledge);
        } else {
            return Err(format!("Path does not exist: {}", path.display()).into());
        }

        Ok(())
    }

    /// Load knowledge from a URL
    pub fn load_knowledge_from_url(&mut self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        // For now, return an error with a helpful message about future implementation
        // This can be implemented later using reqwest or similar HTTP client
        Err(format!("URL knowledge loading not yet implemented: {}", url).into())
    }

    /// Load a single knowledge file
    fn load_single_knowledge_file<P: AsRef<Path>>(&self, path: P) -> Result<Knowledge, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let filename = path.file_stem()
            .ok_or_else(|| format!("Invalid filename: {}", path.display()))?
            .to_str()
            .ok_or_else(|| format!("Invalid UTF-8 in filename: {}", path.display()))?;

        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Unable to read knowledge file {}: {}", path.display(), e))?;

        // Use the same structure as in build.rs
        #[derive(serde::Deserialize)]
        struct YamlKnowledge {
            pub long_name: Option<String>,
            pub ioos_category: Option<String>,
            pub common_variable_names: Option<Vec<String>>,
            pub related_standards: Option<Vec<String>>,
            pub sibling_standards: Option<Vec<String>>,
            pub extra_attrs: Option<std::collections::BTreeMap<String, String>>,
            pub other_units: Option<Vec<String>>,
            pub comments: Option<String>,
            pub qc: Option<std::collections::BTreeMap<String, crate::qartod::static_qc::StaticQc>>,
        }

        let partial_knowledge: YamlKnowledge = serde_yaml_ng::from_str(&contents)
            .map_err(|e| format!("Failed to parse knowledge from {}: {}", path.display(), e))?;

        Ok(Knowledge {
            name: filename.to_string(),
            long_name: partial_knowledge.long_name,
            ioos_category: partial_knowledge.ioos_category,
            common_variable_names: partial_knowledge.common_variable_names.unwrap_or_default(),
            related_standards: partial_knowledge.related_standards.unwrap_or_default(),
            sibling_standards: partial_knowledge.sibling_standards.unwrap_or_default(),
            extra_attrs: partial_knowledge.extra_attrs.unwrap_or_default(),
            other_units: partial_knowledge.other_units.unwrap_or_default(),
            comments: partial_knowledge.comments,
            qc: partial_knowledge.qc,
        })
    }

    /// Load all knowledge files from a directory
    fn load_knowledge_from_directory<P: AsRef<Path>>(&self, path: P) -> Result<Vec<Knowledge>, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let mut knowledge_list = Vec::new();

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if let Some(ext) = file_path.extension() {
                if ext == "yaml" || ext == "yml" {
                    // Skip CF standards file if it exists
                    if let Some(stem) = file_path.file_stem() {
                        if stem == "_cf_standards" {
                            continue;
                        }
                    }

                    match self.load_single_knowledge_file(&file_path) {
                        Ok(knowledge) => knowledge_list.push(knowledge),
                        Err(e) => eprintln!("Warning: Failed to load {}: {}", file_path.display(), e),
                    }
                }
            }
        }

        Ok(knowledge_list)
    }

    /// Load test suites
    pub fn load_test_suites(&mut self) {
        use crate::qartod::test_suites;

        let suites = test_suites();
        for (name, suite) in suites {
            if let Some(standard) = self.standards.get(&name) {
                let new_standard = Standard {
                    qartod: suite,
                    ..standard.clone()
                };
                self.standards.insert(name, new_standard);
            }
        }
    }

    /// Return a set of all known IOOS categories
    pub fn known_ioos_categories(&self) -> HashSet<String> {
        self.standards
            .values()
            .flat_map(|s| s.ioos_category.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_standards() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
    }

    #[test]
    fn can_get_standard() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let pressure = library.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }

    #[test]
    fn can_get_standard_by_alias() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let pressure = library.get("air_pressure_at_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }

    #[test]
    fn can_apply_knowledge() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let pressure = library.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
        assert_eq!(pressure.long_name, None);

        let know = Knowledge {
            name: "air_pressure_at_mean_sea_level".to_string(),
            long_name: Some("Air Pressure at Sea Level".to_string()),
            ..Default::default()
        };

        library.apply_knowledge(vec![know]);

        let updated_pressure = library.get("air_pressure_at_mean_sea_level").unwrap();
        assert_eq!(updated_pressure.name, "air_pressure_at_mean_sea_level");
        assert_eq!(
            updated_pressure.long_name.as_ref().unwrap(),
            "Air Pressure at Sea Level"
        );

        assert_ne!(pressure, updated_pressure);
    }

    #[test]
    fn can_find_by_variable_name() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();
        let know = Knowledge {
            name: "air_pressure_at_mean_sea_level".to_string(),
            long_name: Some("Air Pressure at Sea Level".to_string()),
            common_variable_names: vec!["pressure".to_string()],
            ..Default::default()
        };

        library.apply_knowledge(vec![know]);

        let filtered = library.filter().by_variable_name("pressure");
        let pressure = &filtered.standards[0];
        assert_eq!(pressure.name, "air_pressure_at_mean_sea_level");
    }

    #[test]
    fn can_load_knowledge_from_path() {
        use std::fs;
        use tempfile::tempdir;

        let mut library = StandardsLibrary::default();
        library.load_cf_standards();

        // Create a temporary directory with test knowledge
        let temp_dir = tempdir().unwrap();
        let knowledge_file = temp_dir.path().join("air_temperature.yaml");
        
        let yaml_content = r#"
ioos_category: Test
long_name: Test Air Temperature
common_variable_names:
- test_temp
related_standards:
- air_pressure
other_units:
- F
comments: Test knowledge file
"#;
        fs::write(&knowledge_file, yaml_content).unwrap();

        // Load knowledge from the temporary file
        library.load_knowledge_from_path(&knowledge_file).unwrap();

        // Check that the knowledge was applied
        let standard = library.get("air_temperature").unwrap();
        assert_eq!(standard.long_name.as_ref().unwrap(), "Test Air Temperature");
        assert_eq!(standard.ioos_category.as_ref().unwrap(), "Test");
        assert!(standard.common_variable_names.contains(&"test_temp".to_string()));
        assert_eq!(standard.comments.as_ref().unwrap(), "Test knowledge file");
    }

    #[test]
    fn can_load_knowledge_from_directory() {
        use std::fs;
        use tempfile::tempdir;

        let mut library = StandardsLibrary::default();
        library.load_cf_standards();

        // Create a temporary directory with multiple knowledge files
        let temp_dir = tempdir().unwrap();
        
        let air_temp_file = temp_dir.path().join("air_temperature.yaml");
        let air_temp_content = r#"
ioos_category: Test
long_name: Test Air Temperature
common_variable_names:
- test_temp
"#;
        fs::write(&air_temp_file, air_temp_content).unwrap();

        let air_pressure_file = temp_dir.path().join("air_pressure.yaml");
        let air_pressure_content = r#"
ioos_category: TestPressure
long_name: Test Air Pressure
common_variable_names:
- test_pressure
"#;
        fs::write(&air_pressure_file, air_pressure_content).unwrap();

        // Load knowledge from the directory
        library.load_knowledge_from_path(temp_dir.path()).unwrap();

        // Check that both knowledge files were applied
        let temp_standard = library.get("air_temperature").unwrap();
        assert_eq!(temp_standard.long_name.as_ref().unwrap(), "Test Air Temperature");
        assert_eq!(temp_standard.ioos_category.as_ref().unwrap(), "Test");

        let pressure_standard = library.get("air_pressure").unwrap();
        assert_eq!(pressure_standard.long_name.as_ref().unwrap(), "Test Air Pressure");
        assert_eq!(pressure_standard.ioos_category.as_ref().unwrap(), "TestPressure");
    }

    #[test]
    fn url_loading_returns_not_implemented_error() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();

        let result = library.load_knowledge_from_url("https://example.com/test.yaml");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("URL knowledge loading not yet implemented"));
    }

    #[test]
    fn non_existent_path_returns_error() {
        let mut library = StandardsLibrary::default();
        library.load_cf_standards();

        let result = library.load_knowledge_from_path("/non/existent/path");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Path does not exist"));
    }
}
