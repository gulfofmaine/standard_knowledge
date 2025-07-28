use std::collections::HashMap;

use super::config::{
    ConfigStream, ConfigStreamQartod, FlatLine, GrossRangeTest, RateOfChange, Spike,
};
use super::types::{
    ArgumentType, ArgumentValue, QartodTestTypes, TestArgument, TestSuite, TestSuiteInfo,
};

static FEET_TO_METERS: f64 = 0.3048;

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

    fn scaffold(&self, arguments: HashMap<String, ArgumentValue>) -> Result<ConfigStream, String> {
        println!(
            "Scaffolding water level QARTOD tests for Gulf of Maine with {:?}",
            arguments
        );

        if arguments.is_empty() {
            return Err("No arguments provided for Gulf of Maine water level tests".to_string());
        }

        let mllw = arguments
            .get("mllw")
            .and_then(|v| match v {
                ArgumentValue::Float(f) => Some(*f),
                _ => None,
            })
            .ok_or("Missing required argument: mllw")?;
        let mhhw = arguments
            .get("mhhw")
            .and_then(|v| match v {
                ArgumentValue::Float(f) => Some(*f),
                _ => None,
            })
            .ok_or("Missing required argument: mhhw")?;

        Ok(ConfigStream {
            qartod: ConfigStreamQartod {
                gross_range_test: Some(GrossRangeTest {
                    suspect_span: (mllw - 4.5 * FEET_TO_METERS, mhhw + 6.0 * FEET_TO_METERS),
                    fail_span: (mllw - 4.5 * FEET_TO_METERS, mhhw + 6.0 * FEET_TO_METERS),
                }),
                rate_of_change_test: Some(RateOfChange {
                    rate_threshold: 0.75 * FEET_TO_METERS,
                }),
                spike_test: Some(Spike {
                    suspect_threshold: 0.75 * FEET_TO_METERS,
                    fail_threshold: 1.5 * FEET_TO_METERS,
                }),
                flat_line_test: Some(FlatLine {
                    tolerance: 0.1 * FEET_TO_METERS,
                    suspect_threshold: 2 * 60 * 60,
                    fail_threshold: 3 * 60 * 60,
                }),
                ..Default::default()
            },
        })
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

    fn scaffold(&self, arguments: HashMap<String, ArgumentValue>) -> Result<ConfigStream, String> {
        panic!(
            "Scaffolding water level QARTOD tests for Long Island Sound with {:?}",
            arguments
        );
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
    fn test_gulf_of_maine_scaffold_no_args() {
        let gulf_suite = GulfOfMaineWaterLevel {};
        let args = HashMap::new();
        let config = gulf_suite.scaffold(args);

        assert!(config.is_err());
        assert_eq!(
            config.err().unwrap(),
            "No arguments provided for Gulf of Maine water level tests"
        );
    }

    #[test]
    fn test_gulf_of_maine_missing_arg() {
        let gulf_suite = GulfOfMaineWaterLevel {};
        let mut args = HashMap::new();
        args.insert("mhhw".to_string(), ArgumentValue::Float(1.0)); // Missing mllw

        let config = gulf_suite.scaffold(args);

        assert!(config.is_err());
        assert_eq!(config.err().unwrap(), "Missing required argument: mllw");
    }

    #[test]
    fn test_gulf_of_maine_scaffold_success() {
        let gulf_suite = GulfOfMaineWaterLevel {};
        let mut args = HashMap::new();
        args.insert("mllw".to_string(), ArgumentValue::Float(0.0));
        args.insert("mhhw".to_string(), ArgumentValue::Float(1.0));

        let config = gulf_suite.scaffold(args);
        assert!(config.is_ok());
        let config = config.unwrap();

        let gross_range = GrossRangeTest {
            suspect_span: (-1.3716000000000002, 2.8288),
            fail_span: (-1.3716000000000002, 2.8288),
        };
        let rate_of_change = RateOfChange {
            rate_threshold: 0.22860000000000003,
        };
        let spike = Spike {
            suspect_threshold: 0.22860000000000003,
            fail_threshold: 0.45720000000000005,
        };
        let flat_line = FlatLine {
            tolerance: 0.030480000000000004,
            suspect_threshold: 7200,
            fail_threshold: 10800,
        };

        assert_eq!(
            config.qartod.gross_range_test,
            Some(gross_range),
            "Expected gross range test to match Gulf of Maine specifications"
        );
        assert_eq!(
            config.qartod.rate_of_change_test,
            Some(rate_of_change),
            "Expected rate of change test to match Gulf of Maine specifications"
        );
        assert_eq!(
            config.qartod.spike_test,
            Some(spike),
            "Expected spike test to match Gulf of Maine specifications"
        );
        assert_eq!(
            config.qartod.flat_line_test,
            Some(flat_line),
            "Expected flat line test to match Gulf of Maine specifications"
        );
    }
}
