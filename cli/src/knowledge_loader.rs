use standard_knowledge::{Knowledge, StandardsLibrary};
use std::fs;
use std::path::Path;

/// Load knowledge from a file path (single file or directory)
pub fn load_knowledge_from_path(
    library: &mut StandardsLibrary,
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = path.as_ref();

    if path.is_file() {
        // Load single file
        let knowledge = load_single_knowledge_file(path)?;
        library.apply_knowledge(vec![knowledge]);
    } else if path.is_dir() {
        // Load all YAML files from directory
        let knowledge = load_knowledge_from_directory(path)?;
        library.apply_knowledge(knowledge);
    } else {
        return Err(format!("Path does not exist: {}", path.display()).into());
    }

    Ok(())
}

/// Load knowledge from a URL
pub fn load_knowledge_from_url(
    library: &mut StandardsLibrary,
    url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Download the content from the URL
    let response =
        reqwest::blocking::get(url).map_err(|e| format!("Failed to fetch URL {url}: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("HTTP error {} when fetching {}", response.status(), url).into());
    }

    let contents = response
        .text()
        .map_err(|e| format!("Failed to read response from {url}: {e}"))?;

    // Extract filename from URL for standard name
    let filename = url
        .split('/')
        .next_back()
        .and_then(|f| f.strip_suffix(".yaml").or_else(|| f.strip_suffix(".yml")))
        .unwrap_or("unknown_standard");

    // Parse the YAML content
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
        pub qc: Option<
            std::collections::BTreeMap<String, standard_knowledge::qartod::static_qc::StaticQc>,
        >,
    }

    let partial_knowledge: YamlKnowledge = serde_yaml_ng::from_str(&contents)
        .map_err(|e| format!("Failed to parse YAML from {url}: {e}"))?;

    let knowledge = Knowledge {
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
    };

    library.apply_knowledge(vec![knowledge]);
    Ok(())
}

/// Load a single knowledge file
fn load_single_knowledge_file(path: &Path) -> Result<Knowledge, Box<dyn std::error::Error>> {
    let filename = path
        .file_stem()
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
        pub qc: Option<
            std::collections::BTreeMap<String, standard_knowledge::qartod::static_qc::StaticQc>,
        >,
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
fn load_knowledge_from_directory(
    path: &Path,
) -> Result<Vec<Knowledge>, Box<dyn std::error::Error>> {
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

                match load_single_knowledge_file(&file_path) {
                    Ok(knowledge) => knowledge_list.push(knowledge),
                    Err(e) => eprintln!("Warning: Failed to load {}: {}", file_path.display(), e),
                }
            }
        }
    }

    Ok(knowledge_list)
}
