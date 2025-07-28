use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Time window for temporal constraints
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeWindow {
    pub starting: Option<String>, // ISO 8601 datetime string
    pub ending: Option<String>,   // ISO 8601 datetime string
}

impl Default for TimeWindow {
    fn default() -> Self {
        Self {
            starting: None,
            ending: None,
        }
    }
}

// Context defines the spatial and temporal constraints for tests
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Context {
    pub window: TimeWindow,
    pub region: Option<String>, // GeoJSON or WKT string for simplicity
                                // Note: For hashing, we'll exclude attrs or use a sorted representation
}

impl Context {
    pub fn new(window: TimeWindow, region: Option<String>) -> Self {
        Self { window, region }
    }

    pub fn with_attrs(
        window: TimeWindow,
        region: Option<String>,
        attrs: HashMap<String, String>,
    ) -> (Self, HashMap<String, String>) {
        (Self { window, region }, attrs)
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            window: TimeWindow::default(),
            region: None,
        }
    }
}

// Value type for test arguments
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArgumentValue {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Array(Vec<ArgumentValue>),
}

impl From<String> for ArgumentValue {
    fn from(value: String) -> Self {
        ArgumentValue::String(value)
    }
}

impl From<bool> for ArgumentValue {
    fn from(value: bool) -> Self {
        ArgumentValue::Bool(value)
    }
}

impl From<i64> for ArgumentValue {
    fn from(value: i64) -> Self {
        ArgumentValue::Int(value)
    }
}

impl From<f64> for ArgumentValue {
    fn from(value: f64) -> Self {
        ArgumentValue::Float(value)
    }
}

impl From<Vec<f64>> for ArgumentValue {
    fn from(value: Vec<f64>) -> Self {
        ArgumentValue::Array(value.into_iter().map(ArgumentValue::Float).collect())
    }
}

// Represents a configured test call
#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub stream_id: String,
    pub module: String,
    pub method: String,
    pub kwargs: HashMap<String, ArgumentValue>,
    pub context: Context,
    pub attrs: HashMap<String, String>,
}

impl Call {
    pub fn new(
        stream_id: String,
        module: String,
        method: String,
        kwargs: HashMap<String, ArgumentValue>,
        context: Context,
    ) -> Self {
        Self {
            stream_id,
            module,
            method,
            kwargs,
            context,
            attrs: HashMap::new(),
        }
    }

    pub fn method_path(&self) -> String {
        format!("{}.{}", self.module, self.method)
    }

    pub fn config(&self) -> HashMap<String, HashMap<String, HashMap<String, ArgumentValue>>> {
        let mut config = HashMap::new();
        let mut method_config = HashMap::new();
        method_config.insert(self.method.clone(), self.kwargs.clone());
        config.insert(self.module.clone(), method_config);
        config
    }
}

// Main Config struct - equivalent to Python's Config class
#[derive(Clone, Debug)]
pub struct Config {
    calls: Vec<Call>,
}

impl Config {
    pub fn new() -> Self {
        Self { calls: Vec::new() }
    }

    pub fn from_calls(calls: Vec<Call>) -> Self {
        Self { calls }
    }

    pub fn calls(&self) -> &[Call] {
        &self.calls
    }

    pub fn stream_ids(&self) -> Vec<String> {
        let mut stream_ids = Vec::new();
        let mut seen = HashMap::new();

        for call in &self.calls {
            if !seen.contains_key(&call.stream_id) {
                seen.insert(call.stream_id.clone(), true);
                stream_ids.push(call.stream_id.clone());
            }
        }

        stream_ids
    }

    pub fn calls_by_stream_id(&self, stream_id: &str) -> Vec<&Call> {
        self.calls
            .iter()
            .filter(|call| call.stream_id == stream_id)
            .collect()
    }

    pub fn contexts(&self) -> HashMap<Context, Vec<&Call>> {
        let mut contexts = HashMap::new();

        for call in &self.calls {
            contexts
                .entry(call.context.clone())
                .or_insert_with(Vec::new)
                .push(call);
        }

        contexts
    }

    pub fn add_call(&mut self, call: Call) {
        self.calls.push(call);
    }

    pub fn add_calls(&mut self, calls: Vec<Call>) {
        self.calls.extend(calls);
    }

    pub fn has_test(&self, stream_id: &str, method_path: &str) -> Option<&Call> {
        self.calls
            .iter()
            .find(|call| call.stream_id == stream_id && call.method_path() == method_path)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

// Builder for easier Config creation
pub struct ConfigBuilder {
    calls: Vec<Call>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self { calls: Vec::new() }
    }

    pub fn add_test(
        mut self,
        stream_id: &str,
        module: &str,
        method: &str,
        kwargs: HashMap<String, ArgumentValue>,
    ) -> Self {
        let call = Call::new(
            stream_id.to_string(),
            module.to_string(),
            method.to_string(),
            kwargs,
            Context::default(),
        );
        self.calls.push(call);
        self
    }

    pub fn add_test_with_context(
        mut self,
        stream_id: &str,
        module: &str,
        method: &str,
        kwargs: HashMap<String, ArgumentValue>,
        context: Context,
    ) -> Self {
        let call = Call::new(
            stream_id.to_string(),
            module.to_string(),
            method.to_string(),
            kwargs,
            context,
        );
        self.calls.push(call);
        self
    }

    pub fn build(self) -> Config {
        Config::from_calls(self.calls)
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// YAML loading and parsing functionality
impl Config {
    /// Load a Config from a YAML file path
    pub fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        Self::from_yaml_str(&content)
    }

    /// Load a Config from a YAML string
    pub fn from_yaml_str(yaml_content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Try to parse as a multi-context configuration first
        if let Ok(contexts_config) = serde_yaml_ng::from_str::<ContextsConfig>(yaml_content) {
            let mut config = Config::new();
            for context_config in contexts_config.contexts {
                let calls = context_config.to_calls();
                config.add_calls(calls);
            }
            return Ok(config);
        }

        // Try to parse as a single context configuration
        if let Ok(context_config) = serde_yaml_ng::from_str::<ContextConfig>(yaml_content) {
            let calls = context_config.to_calls();
            return Ok(Config::from_calls(calls));
        }

        // Try to parse as a stream configuration (no context)
        if let Ok(stream_config) = serde_yaml_ng::from_str::<
            HashMap<String, HashMap<String, HashMap<String, serde_yaml_ng::Value>>>,
        >(yaml_content)
        {
            let mut config = Config::new();
            let context = Context::default();

            for (stream_id, packages) in stream_config {
                for (package, methods) in packages {
                    for (method, kwargs_value) in methods {
                        // Convert serde_yaml_ng::Value to ArgumentValue
                        let kwargs = convert_yaml_value_to_argument_map(kwargs_value)?;

                        let call = Call::new(
                            stream_id.clone(),
                            package.clone(),
                            method.clone(),
                            kwargs,
                            context.clone(),
                        );
                        config.add_call(call);
                    }
                }
            }
            return Ok(config);
        }

        Err("Failed to parse YAML as any known configuration format".into())
    }
}

// Context-specific configuration - equivalent to Python's ContextConfig
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<serde_yaml_ng::Value>, // Keep as Value to handle GeoJSON or string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<TimeWindow>,
    pub streams: HashMap<String, HashMap<String, HashMap<String, serde_yaml_ng::Value>>>, // Use Value for flexibility
    #[serde(default)]
    pub attrs: HashMap<String, String>,
}

impl ContextConfig {
    pub fn new() -> Self {
        Self {
            region: None,
            window: None,
            streams: HashMap::new(),
            attrs: HashMap::new(),
        }
    }

    pub fn to_calls(&self) -> Vec<Call> {
        let mut calls = Vec::new();

        // Convert region to string representation
        let region_str = self.region.as_ref().map(|r| match r {
            serde_yaml_ng::Value::String(s) => s.clone(),
            serde_yaml_ng::Value::Null => "null".to_string(),
            _ => serde_yaml_ng::to_string(r).unwrap_or_else(|_| "unknown".to_string()),
        });

        let context = Context {
            window: self.window.clone().unwrap_or_default(),
            region: region_str,
        };

        for (stream_id, packages) in &self.streams {
            for (package, methods) in packages {
                for (method, kwargs_value) in methods {
                    // Convert serde_yaml_ng::Value to HashMap<String, ArgumentValue>
                    let kwargs = match convert_yaml_value_to_argument_map(kwargs_value.clone()) {
                        Ok(map) => map,
                        Err(_) => {
                            // If conversion fails, create a simple map
                            let mut map = HashMap::new();
                            if let Ok(arg_val) =
                                convert_yaml_value_to_argument_value(kwargs_value.clone())
                            {
                                map.insert("value".to_string(), arg_val);
                            }
                            map
                        }
                    };

                    let call = Call::new(
                        stream_id.clone(),
                        package.clone(),
                        method.clone(),
                        kwargs,
                        context.clone(),
                    );
                    calls.push(call);
                }
            }
        }

        calls
    }
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self::new()
    }
}

// Helper struct for parsing multi-context YAML configs
#[derive(Debug, Serialize, Deserialize)]
struct ContextsConfig {
    contexts: Vec<ContextConfig>,
}

// Helper function to convert serde_yaml_ng::Value to HashMap<String, ArgumentValue>
fn convert_yaml_value_to_argument_map(
    value: serde_yaml_ng::Value,
) -> Result<HashMap<String, ArgumentValue>, Box<dyn std::error::Error>> {
    match value {
        serde_yaml_ng::Value::Mapping(map) => {
            let mut result = HashMap::new();
            for (k, v) in map {
                let key = match k {
                    serde_yaml_ng::Value::String(s) => s,
                    _ => return Err("Non-string key in YAML mapping".into()),
                };
                let arg_value = convert_yaml_value_to_argument_value(v)?;
                result.insert(key, arg_value);
            }
            Ok(result)
        }
        _ => {
            // If it's not a mapping, create a single entry
            let mut result = HashMap::new();
            let arg_value = convert_yaml_value_to_argument_value(value)?;
            result.insert("value".to_string(), arg_value);
            Ok(result)
        }
    }
}

// Helper function to convert serde_yaml_ng::Value to ArgumentValue
fn convert_yaml_value_to_argument_value(
    value: serde_yaml_ng::Value,
) -> Result<ArgumentValue, Box<dyn std::error::Error>> {
    match value {
        serde_yaml_ng::Value::String(s) => Ok(ArgumentValue::String(s)),
        serde_yaml_ng::Value::Bool(b) => Ok(ArgumentValue::Bool(b)),
        serde_yaml_ng::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(ArgumentValue::Int(i))
            } else if let Some(f) = n.as_f64() {
                Ok(ArgumentValue::Float(f))
            } else {
                Err("Invalid number format".into())
            }
        }
        serde_yaml_ng::Value::Sequence(seq) => {
            let mut result = Vec::new();
            for item in seq {
                result.push(convert_yaml_value_to_argument_value(item)?);
            }
            Ok(ArgumentValue::Array(result))
        }
        serde_yaml_ng::Value::Null => Ok(ArgumentValue::String("null".to_string())),
        _ => Err("Unsupported YAML value type".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let mut kwargs = HashMap::new();
        kwargs.insert(
            "suspect_span".to_string(),
            ArgumentValue::Array(vec![ArgumentValue::Float(-2.0), ArgumentValue::Float(5.0)]),
        );

        let config = ConfigBuilder::new()
            .add_test("water_level", "qartod", "gross_range_test", kwargs)
            .build();

        assert_eq!(config.calls().len(), 1);
        assert_eq!(config.stream_ids(), vec!["water_level"]);
    }

    #[test]
    fn test_context_config() {
        let mut context_config = ContextConfig::new();
        context_config.window = Some(TimeWindow {
            starting: Some("2020-01-01T00:00:00Z".to_string()),
            ending: Some("2020-12-31T23:59:59Z".to_string()),
        });

        let mut streams = HashMap::new();
        let mut qartod_tests = HashMap::new();
        let mut gross_range = HashMap::new();
        gross_range.insert(
            "gross_range_test".to_string(),
            serde_yaml_ng::Value::Sequence(vec![
                serde_yaml_ng::Value::Number(serde_yaml_ng::Number::from(-2.0)),
                serde_yaml_ng::Value::Number(serde_yaml_ng::Number::from(5.0)),
            ]),
        );
        qartod_tests.insert("qartod".to_string(), gross_range);
        streams.insert("water_level".to_string(), qartod_tests);
        context_config.streams = streams;

        let calls = context_config.to_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].stream_id, "water_level");
        assert_eq!(calls[0].module, "qartod");
    }

    #[test]
    fn test_argument_value_conversions() {
        let string_val: ArgumentValue = "test".to_string().into();
        assert!(matches!(string_val, ArgumentValue::String(_)));

        let bool_val: ArgumentValue = true.into();
        assert!(matches!(bool_val, ArgumentValue::Bool(true)));

        let int_val: ArgumentValue = 42i64.into();
        assert!(matches!(int_val, ArgumentValue::Int(42)));

        let float_val: ArgumentValue = 3.14f64.into();
        assert!(matches!(float_val, ArgumentValue::Float(_)));

        let array_val: ArgumentValue = vec![1.0, 2.0, 3.0].into();
        assert!(matches!(array_val, ArgumentValue::Array(_)));
    }

    // Integration tests for YAML loading
    #[test]
    fn test_load_context_config_yaml() {
        let yaml_path = "src/qartod/examples/context_config.yaml";
        let config = Config::from_yaml_file(yaml_path);

        assert!(
            config.is_ok(),
            "Failed to load context_config.yaml: {:?}",
            config.err()
        );
        let config = config.unwrap();

        // Should have calls for variable1 and variable2
        assert_eq!(config.calls().len(), 2);

        let stream_ids = config.stream_ids();
        assert!(stream_ids.contains(&"variable1".to_string()));
        assert!(stream_ids.contains(&"variable2".to_string()));

        // Check that the time window was parsed correctly
        let contexts = config.contexts();
        assert_eq!(contexts.len(), 1);
        let context = contexts.keys().next().unwrap();
        assert_eq!(
            context.window.starting,
            Some("2020-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            context.window.ending,
            Some("2020-04-01T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_load_context_lists_yaml() {
        let yaml_path = "src/qartod/examples/context_lists.yaml";
        let config = Config::from_yaml_file(yaml_path);

        assert!(
            config.is_ok(),
            "Failed to load context_lists.yaml: {:?}",
            config.err()
        );
        let config = config.unwrap();

        // Should have calls from multiple contexts (2 contexts × 2 variables each = 4 calls)
        assert_eq!(config.calls().len(), 4);

        let contexts = config.contexts();
        assert_eq!(contexts.len(), 2, "Should have 2 different contexts");

        // Verify that all contexts have the same time window
        for context in contexts.keys() {
            assert_eq!(
                context.window.starting,
                Some("2020-01-01T00:00:00+00:00".to_string())
            );
            assert_eq!(
                context.window.ending,
                Some("2020-04-01T00:00:00+00:00".to_string())
            );
        }
    }

    #[test]
    fn test_load_stream_config_yaml() {
        let yaml_path = "src/qartod/examples/stream_config.yaml";
        let config = Config::from_yaml_file(yaml_path);

        assert!(
            config.is_ok(),
            "Failed to load stream_config.yaml: {:?}",
            config.err()
        );
        let config = config.unwrap();

        // Should have calls for variable1
        assert!(!config.calls().is_empty());
        assert_eq!(config.stream_ids(), vec!["variable1"]);

        // Check that we have the expected test methods
        let calls = config.calls();
        let method_names: Vec<&String> = calls.iter().map(|c| &c.method).collect();
        assert!(method_names.contains(&&"gross_range_test".to_string()));
        assert!(method_names.contains(&&"location_test".to_string()));
    }

    #[test]
    fn test_load_context_config_region_yaml() {
        let yaml_path = "src/qartod/examples/context_config_region.yaml";
        let config = Config::from_yaml_file(yaml_path);

        assert!(
            config.is_ok(),
            "Failed to load context_config_region.yaml: {:?}",
            config.err()
        );
        let config = config.unwrap();

        // Should have calls for variable1 and variable2
        assert_eq!(config.calls().len(), 2);

        // Check that the region was parsed
        let contexts = config.contexts();
        assert_eq!(contexts.len(), 1);
        let context = contexts.keys().next().unwrap();
        assert!(context.region.is_some());
        assert_eq!(context.region, Some("something".to_string()));
    }

    #[test]
    fn test_load_context_config2_yaml() {
        let yaml_path = "src/qartod/examples/context_config2.yaml";
        let config = Config::from_yaml_file(yaml_path);

        assert!(
            config.is_ok(),
            "Failed to load context_config2.yaml: {:?}",
            config.err()
        );
        let config = config.unwrap();

        // Should have calls for variable1 and variable2
        assert_eq!(config.calls().len(), 2);

        let stream_ids = config.stream_ids();
        assert!(stream_ids.contains(&"variable1".to_string()));
        assert!(stream_ids.contains(&"variable2".to_string()));

        // This config has no window/region, so should use default context
        let contexts = config.contexts();
        assert_eq!(contexts.len(), 1);
        let context = contexts.keys().next().unwrap();
        assert_eq!(context.window.starting, None);
        assert_eq!(context.window.ending, None);
        assert_eq!(context.region, None);
    }

    #[test]
    fn test_yaml_parsing_with_null_values() {
        let yaml_content = r#"
region: null
window:
    starting: 2020-01-01T00:00:00Z
    ending: null
streams:
    test_variable:
        qartod:
            test_method:
                param1: null
                param2: [1, null, 3]
"#;

        let config = Config::from_yaml_str(yaml_content);
        assert!(
            config.is_ok(),
            "Failed to parse YAML with null values: {:?}",
            config.err()
        );

        let config = config.unwrap();
        assert_eq!(config.calls().len(), 1);

        let call = &config.calls()[0];
        assert_eq!(call.stream_id, "test_variable");
        assert_eq!(call.method, "test_method");

        // Check that null values were handled
        assert!(call.kwargs.contains_key("param1"));
        assert!(call.kwargs.contains_key("param2"));
    }

    #[test]
    fn test_yaml_error_handling() {
        let invalid_yaml = "invalid: yaml: content: [unclosed";
        let result = Config::from_yaml_str(invalid_yaml);
        assert!(result.is_err(), "Should fail to parse invalid YAML");
    }

    #[test]
    fn test_config_roundtrip() {
        // Create a config programmatically
        let mut kwargs = HashMap::new();
        kwargs.insert(
            "suspect_span".to_string(),
            ArgumentValue::Array(vec![ArgumentValue::Float(1.0), ArgumentValue::Float(11.0)]),
        );
        kwargs.insert(
            "fail_span".to_string(),
            ArgumentValue::Array(vec![ArgumentValue::Float(0.0), ArgumentValue::Float(12.0)]),
        );

        let original_config = ConfigBuilder::new()
            .add_test("variable1", "qartod", "gross_range_test", kwargs)
            .build();

        // Verify the config structure
        assert_eq!(original_config.calls().len(), 1);
        assert_eq!(original_config.stream_ids(), vec!["variable1"]);

        let call = &original_config.calls()[0];
        assert_eq!(call.stream_id, "variable1");
        assert_eq!(call.module, "qartod");
        assert_eq!(call.method, "gross_range_test");
        assert!(call.kwargs.contains_key("suspect_span"));
        assert!(call.kwargs.contains_key("fail_span"));
    }
}
