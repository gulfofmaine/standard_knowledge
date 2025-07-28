use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct GrossRangeTest {
    pub suspect_span: (f64, f64),
    pub fail_span: (f64, f64),
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct LocationTest {
    bbox: (f64, f64, f64, f64), // (min_lon, min_lat, max_lon, max_lat)
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct RateOfChange {
    pub rate_threshold: f64,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Spike {
    pub suspect_threshold: f64,
    pub fail_threshold: f64,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct FlatLine {
    pub tolerance: f64,
    pub suspect_threshold: isize,
    pub fail_threshold: isize,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct ConfigStreamQartod {
    pub gross_range_test: Option<GrossRangeTest>,
    pub location_test: Option<LocationTest>,
    pub rate_of_change_test: Option<RateOfChange>,
    pub spike_test: Option<Spike>,
    pub flat_line_test: Option<FlatLine>,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct ConfigStream {
    pub qartod: ConfigStreamQartod,
}

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

pub struct ConfigContext {
    window: Option<TimeWindow>,
    region: Option<String>,
    streams: HashMap<String, ConfigStream>,
}

pub struct Config {
    contexts: Vec<ConfigContext>,
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_config_builder() {
//         let mut kwargs = HashMap::new();
//         kwargs.insert(
//             "suspect_span".to_string(),
//             ArgumentValue::Array(vec![ArgumentValue::Float(-2.0), ArgumentValue::Float(5.0)]),
//         );

//         let config = ConfigBuilder::new()
//             .add_test("water_level", "qartod", "gross_range_test", kwargs)
//             .build();

//         assert_eq!(config.calls().len(), 1);
//         assert_eq!(config.stream_ids(), vec!["water_level"]);
//     }

//     #[test]
//     fn test_context_config() {
//         let mut context_config = ContextConfig::new();
//         context_config.window = Some(TimeWindow {
//             starting: Some("2020-01-01T00:00:00Z".to_string()),
//             ending: Some("2020-12-31T23:59:59Z".to_string()),
//         });

//         let mut streams = HashMap::new();
//         let mut qartod_tests = HashMap::new();
//         let mut gross_range = HashMap::new();
//         gross_range.insert(
//             "gross_range_test".to_string(),
//             serde_yaml_ng::Value::Sequence(vec![
//                 serde_yaml_ng::Value::Number(serde_yaml_ng::Number::from(-2.0)),
//                 serde_yaml_ng::Value::Number(serde_yaml_ng::Number::from(5.0)),
//             ]),
//         );
//         qartod_tests.insert("qartod".to_string(), gross_range);
//         streams.insert("water_level".to_string(), qartod_tests);
//         context_config.streams = streams;

//         let calls = context_config.to_calls();
//         assert_eq!(calls.len(), 1);
//         assert_eq!(calls[0].stream_id, "water_level");
//         assert_eq!(calls[0].module, "qartod");
//     }

//     #[test]
//     fn test_argument_value_conversions() {
//         let string_val: ArgumentValue = "test".to_string().into();
//         assert!(matches!(string_val, ArgumentValue::String(_)));

//         let bool_val: ArgumentValue = true.into();
//         assert!(matches!(bool_val, ArgumentValue::Bool(true)));

//         let int_val: ArgumentValue = 42i64.into();
//         assert!(matches!(int_val, ArgumentValue::Int(42)));

//         let float_val: ArgumentValue = 3.14f64.into();
//         assert!(matches!(float_val, ArgumentValue::Float(_)));

//         let array_val: ArgumentValue = vec![1.0, 2.0, 3.0].into();
//         assert!(matches!(array_val, ArgumentValue::Array(_)));
//     }

//     // Integration tests for YAML loading
//     #[test]
//     fn test_load_context_config_yaml() {
//         let yaml_path = "src/qartod/examples/context_config.yaml";
//         let config = Config::from_yaml_file(yaml_path);

//         assert!(
//             config.is_ok(),
//             "Failed to load context_config.yaml: {:?}",
//             config.err()
//         );
//         let config = config.unwrap();

//         // Should have calls for variable1 and variable2
//         assert_eq!(config.calls().len(), 2);

//         let stream_ids = config.stream_ids();
//         assert!(stream_ids.contains(&"variable1".to_string()));
//         assert!(stream_ids.contains(&"variable2".to_string()));

//         // Check that the time window was parsed correctly
//         let contexts = config.contexts();
//         assert_eq!(contexts.len(), 1);
//         let context = contexts.keys().next().unwrap();
//         assert_eq!(
//             context.window.starting,
//             Some("2020-01-01T00:00:00Z".to_string())
//         );
//         assert_eq!(
//             context.window.ending,
//             Some("2020-04-01T00:00:00Z".to_string())
//         );
//     }

//     #[test]
//     fn test_load_context_lists_yaml() {
//         let yaml_path = "src/qartod/examples/context_lists.yaml";
//         let config = Config::from_yaml_file(yaml_path);

//         assert!(
//             config.is_ok(),
//             "Failed to load context_lists.yaml: {:?}",
//             config.err()
//         );
//         let config = config.unwrap();

//         // Should have calls from multiple contexts (2 contexts × 2 variables each = 4 calls)
//         assert_eq!(config.calls().len(), 4);

//         let contexts = config.contexts();
//         assert_eq!(contexts.len(), 2, "Should have 2 different contexts");

//         // Verify that all contexts have the same time window
//         for context in contexts.keys() {
//             assert_eq!(
//                 context.window.starting,
//                 Some("2020-01-01T00:00:00+00:00".to_string())
//             );
//             assert_eq!(
//                 context.window.ending,
//                 Some("2020-04-01T00:00:00+00:00".to_string())
//             );
//         }
//     }

//     #[test]
//     fn test_load_stream_config_yaml() {
//         let yaml_path = "src/qartod/examples/stream_config.yaml";
//         let config = Config::from_yaml_file(yaml_path);

//         assert!(
//             config.is_ok(),
//             "Failed to load stream_config.yaml: {:?}",
//             config.err()
//         );
//         let config = config.unwrap();

//         // Should have calls for variable1
//         assert!(!config.calls().is_empty());
//         assert_eq!(config.stream_ids(), vec!["variable1"]);

//         // Check that we have the expected test methods
//         let calls = config.calls();
//         let method_names: Vec<&String> = calls.iter().map(|c| &c.method).collect();
//         assert!(method_names.contains(&&"gross_range_test".to_string()));
//         assert!(method_names.contains(&&"location_test".to_string()));

//         panic!("Calls {:?}", calls);
//     }

//     #[test]
//     fn test_load_context_config_region_yaml() {
//         let yaml_path = "src/qartod/examples/context_config_region.yaml";
//         let config = Config::from_yaml_file(yaml_path);

//         assert!(
//             config.is_ok(),
//             "Failed to load context_config_region.yaml: {:?}",
//             config.err()
//         );
//         let config = config.unwrap();

//         // Should have calls for variable1 and variable2
//         assert_eq!(config.calls().len(), 2);

//         // Check that the region was parsed
//         let contexts = config.contexts();
//         assert_eq!(contexts.len(), 1);
//         let context = contexts.keys().next().unwrap();
//         assert!(context.region.is_some());
//         assert_eq!(context.region, Some("something".to_string()));
//     }

//     #[test]
//     fn test_load_context_config2_yaml() {
//         let yaml_path = "src/qartod/examples/context_config2.yaml";
//         let config = Config::from_yaml_file(yaml_path);

//         assert!(
//             config.is_ok(),
//             "Failed to load context_config2.yaml: {:?}",
//             config.err()
//         );
//         let config = config.unwrap();

//         // Should have calls for variable1 and variable2
//         assert_eq!(config.calls().len(), 2);

//         let stream_ids = config.stream_ids();
//         assert!(stream_ids.contains(&"variable1".to_string()));
//         assert!(stream_ids.contains(&"variable2".to_string()));

//         // This config has no window/region, so should use default context
//         let contexts = config.contexts();
//         assert_eq!(contexts.len(), 1);
//         let context = contexts.keys().next().unwrap();
//         assert_eq!(context.window.starting, None);
//         assert_eq!(context.window.ending, None);
//         assert_eq!(context.region, None);
//     }

//     #[test]
//     fn test_yaml_parsing_with_null_values() {
//         let yaml_content = r#"
// region: null
// window:
//     starting: 2020-01-01T00:00:00Z
//     ending: null
// streams:
//     test_variable:
//         qartod:
//             test_method:
//                 param1: null
//                 param2: [1, null, 3]
// "#;

//         let config = Config::from_yaml_str(yaml_content);
//         assert!(
//             config.is_ok(),
//             "Failed to parse YAML with null values: {:?}",
//             config.err()
//         );

//         let config = config.unwrap();
//         assert_eq!(config.calls().len(), 1);

//         let call = &config.calls()[0];
//         assert_eq!(call.stream_id, "test_variable");
//         assert_eq!(call.method, "test_method");

//         // Check that null values were handled
//         assert!(call.kwargs.contains_key("param1"));
//         assert!(call.kwargs.contains_key("param2"));
//     }

//     #[test]
//     fn test_yaml_error_handling() {
//         let invalid_yaml = "invalid: yaml: content: [unclosed";
//         let result = Config::from_yaml_str(invalid_yaml);
//         assert!(result.is_err(), "Should fail to parse invalid YAML");
//     }

//     #[test]
//     fn test_config_roundtrip() {
//         // Create a config programmatically
//         let mut kwargs = HashMap::new();
//         kwargs.insert(
//             "suspect_span".to_string(),
//             ArgumentValue::Array(vec![ArgumentValue::Float(1.0), ArgumentValue::Float(11.0)]),
//         );
//         kwargs.insert(
//             "fail_span".to_string(),
//             ArgumentValue::Array(vec![ArgumentValue::Float(0.0), ArgumentValue::Float(12.0)]),
//         );

//         let original_config = ConfigBuilder::new()
//             .add_test("variable1", "qartod", "gross_range_test", kwargs)
//             .build();

//         // Verify the config structure
//         assert_eq!(original_config.calls().len(), 1);
//         assert_eq!(original_config.stream_ids(), vec!["variable1"]);

//         let call = &original_config.calls()[0];
//         assert_eq!(call.stream_id, "variable1");
//         assert_eq!(call.module, "qartod");
//         assert_eq!(call.method, "gross_range_test");
//         assert!(call.kwargs.contains_key("suspect_span"));
//         assert!(call.kwargs.contains_key("fail_span"));
//     }
// }
