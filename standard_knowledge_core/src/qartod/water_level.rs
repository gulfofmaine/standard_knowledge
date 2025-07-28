use std::collections::HashMap;

use super::config::{ArgumentValue, Call, Config, Context};
use super::types::{ArgumentType, QartodTestTypes, TestArgument, TestSuite, TestSuiteInfo};

static GULF_OF_MAINE: &str = r#"
Testing range suggestions developed in coordination with
Hannah Baranes for the Gulf of Maine region.
"#;

#[derive(Debug, Clone)]
struct GulfOfMaineWaterLevel {}

impl TestSuite for GulfOfMaineWaterLevel {
    fn info(&self) -> TestSuiteInfo {
        TestSuiteInfo {
            name: "Gulf of Maine".to_string(),
            slug: "gulf_of_maine".to_string(),
            summary:
                "Water level tests for stations in the Gulf of Maine developed by Hannah Baranes"
                    .to_string(),
            description: GULF_OF_MAINE.to_string(),
            arguments: HashMap::from([(
                "mllw".to_string(),
                TestArgument {
                    argument_type: ArgumentType::Float,
                    description: "Mean lower low water elevation in NAVD 88 meters".to_string(),
                    required: true,
                },
            )]),
            test_types: vec![
                QartodTestTypes::GrossRange,
                QartodTestTypes::Spike,
                QartodTestTypes::RateOfChange,
                QartodTestTypes::FlatLine,
            ],
        }
    }

    fn scaffold(&self, arguments: HashMap<String, ArgumentValue>) -> Config {
        println!(
            "Scaffolding water level QARTOD tests for Gulf of Maine with {:?}",
            arguments
        );

        // Create a sample config with common water level tests
        let mut config = Config::new();
        let context = Context::default();

        // Add gross range test
        let mut gross_range_kwargs = HashMap::new();
        gross_range_kwargs.insert(
            "suspect_span".to_string(),
            ArgumentValue::Array(vec![ArgumentValue::Float(-2.0), ArgumentValue::Float(5.0)]),
        );
        gross_range_kwargs.insert(
            "fail_span".to_string(),
            ArgumentValue::Array(vec![ArgumentValue::Float(-3.0), ArgumentValue::Float(6.0)]),
        );

        config.add_call(Call::new(
            "water_level".to_string(),
            "qartod".to_string(),
            "gross_range_test".to_string(),
            gross_range_kwargs,
            context.clone(),
        ));

        // Add spike test
        let mut spike_kwargs = HashMap::new();
        spike_kwargs.insert("suspect_threshold".to_string(), ArgumentValue::Float(0.5));
        spike_kwargs.insert("fail_threshold".to_string(), ArgumentValue::Float(1.0));

        config.add_call(Call::new(
            "water_level".to_string(),
            "qartod".to_string(),
            "spike_test".to_string(),
            spike_kwargs,
            context.clone(),
        ));

        config
    }
}

#[derive(Debug, Clone)]
struct LongIslandSoundWaterLevel {}

impl TestSuite for LongIslandSoundWaterLevel {
    fn info(&self) -> TestSuiteInfo {
        TestSuiteInfo {
            name: "Long Island Sound".to_string(),
            slug: "long_island_sound".to_string(),
            summary: "Water level tests for stations in Long Island Sound".to_string(),
            description: "Water level tests for Long Island Sound by Anna".to_string(),
            arguments: HashMap::from([(
                "mllw".to_string(),
                TestArgument {
                    argument_type: ArgumentType::Float,
                    description: "Mean lower low water elevation in NAVD 88 meters".to_string(),
                    required: true,
                },
            )]),
            test_types: vec![
                QartodTestTypes::GrossRange,
                QartodTestTypes::Spike,
                QartodTestTypes::RateOfChange,
                QartodTestTypes::FlatLine,
            ],
        }
    }

    fn scaffold(&self, arguments: HashMap<String, ArgumentValue>) -> Config {
        println!(
            "Scaffolding water level QARTOD tests for Long Island Sound with {:?}",
            arguments
        );

        // Create a sample config with common water level tests for Long Island Sound
        let mut config = Config::new();
        let context = Context::default();

        // Add gross range test with different thresholds for Long Island Sound
        let mut gross_range_kwargs = HashMap::new();
        gross_range_kwargs.insert(
            "suspect_span".to_string(),
            ArgumentValue::Array(vec![ArgumentValue::Float(-1.5), ArgumentValue::Float(4.0)]),
        );
        gross_range_kwargs.insert(
            "fail_span".to_string(),
            ArgumentValue::Array(vec![ArgumentValue::Float(-2.5), ArgumentValue::Float(5.0)]),
        );

        config.add_call(Call::new(
            "water_level".to_string(),
            "qartod".to_string(),
            "gross_range_test".to_string(),
            gross_range_kwargs,
            context.clone(),
        ));

        config
    }
}

pub fn water_level_test_suites() -> HashMap<String, Vec<Box<dyn TestSuite>>> {
    let mut suites: HashMap<String, Vec<Box<dyn TestSuite>>> = HashMap::new();

    // Add Gulf of Maine water level tests
    suites.insert(
        "sea_surface_height_above_geopotential_datum".to_string(),
        vec![
            Box::new(GulfOfMaineWaterLevel {}),
            Box::new(LongIslandSoundWaterLevel {}),
        ],
    );

    suites
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_object_compatibility() {
        let gulf_of_maine: Box<dyn TestSuite> = Box::new(GulfOfMaineWaterLevel {});
        let _info = gulf_of_maine.info();

        let long_island: Box<dyn TestSuite> = Box::new(LongIslandSoundWaterLevel {});
        let _info = long_island.info();
    }

    #[test]
    fn test_test_suite_scaffold() {
        let gulf_suite = GulfOfMaineWaterLevel {};
        let args = HashMap::new();
        let config = gulf_suite.scaffold(args);

        // Should have generated some test calls
        assert!(!config.calls().is_empty());
        assert_eq!(config.stream_ids(), vec!["water_level"]);
    }
}
