use std::collections::HashMap;

use super::config::{
    ConfigStream, ConfigStreamQartod, FlatLine, GrossRangeTest, RateOfChange, Spike,
};
use super::types::{
    ArgumentType, ArgumentValue, QartodTestTypes, TestArgument, TestSuite, TestSuiteInfo,
};

static FEET_TO_METERS: f64 = 0.3048;

static GULF_OF_MAINE: &str = r#"
### Gross range test configuration for Gulf of Maine (not New England Shelf)

#### Suspect Limits

For stations with tidal datums (might not want this approach because it will always take a while to get tidal datums, and tidal datums change):
- Upper limit of range: MHHW + 6 ft
- Lower limit of range: MLLW – 4.5 ft



For stations without tidal datums:
- If there are no tidal datums because the station was just installed: use VDatum to get MHHW and MLLW relative to navd88_meters at a point close to the sensor, and use the same upper and lower limits
    - Note: if it’s a station with river influence (like Bath), it might require some local expertise to set the limits. A solid approach is just taking the HW and LW measured over the course of the first week, and using something like HW + 10 ft and LW – 10 ft to be conservative
- If there are no tidal datums because the sensor bottoms out at low tide:
    - Lower limit: Use the dry bottom elevation
    - Upper limit: Use VDatum MHHW + 6 ft


#### Fail upper and lower limits
- Upper limit: distance to water is less than whatever the minimum sensing range is
- Lower limit: either hard bottom (if it’s a site that bottoms out at LW, or if we have a depth measurement at the site), or distance to water = maximum of sensing range

#### Notes

Top recorded water levels, in ft MHHW (and year)
- Gulf of Maine
    - Eastport: 5.07 (2020)
    - Bar Harbor: 4.43 (2024)
    - Portland: 4.67 (2024)
    - Boston: 4.89 (2018)
- New England Shelf
    - Chatham, MA: 4.28 (2014)
    - Newport, RI: 9.45 (1938)
    -New London, CT: 7.53 (1938)

Lowest navd88_meters
- Eastport: -3.46 ft MLLW  (this will have the largest variability)

### Rate of change test. Input as a rate.

- Suspect: 0.75 feet per 6 minutes
- Fail: 1 foot per 6 minutes

Rationale: max rate of change from tides in Eastport is 5.3 ft per hour (midtide on 1/13/2024), or ~0.5 ft per 6 minutes. Add 0.25 feet for a sustained wind-driven increase in water level.

May want to adjust this so it’s dependent on tidal range

### Spike test: Input as a magnitude that’s checked across a measurement and the two adjacent measurements.

Maybe default to same as rate of change test?

### Flat line test: If there’s some lack of variance over some amount of time, mark as suspect/fail

Suspect/Fail = how long do subsequent values stay within that threshold before it’s considered flat? (input as a time)

For example, if all measurements over the past 4 hours are within 10 cm of each other, fail the flatline test (then tolerance = 10 cm, and time = 4 hours)

When a sensor flatlines, the system voltage and temperature sensor may still be causing variation

Let’s start with 0.1 feet over 2 hours for suspect, and 0.1 feet over 3 hours for fail.

Rationale: During neap tides in Portland, you could see as little as +/- 0.25 ft per hour of variation in the 2 hours around slack tide (HW or LW)
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
            arguments: HashMap::from([
                (
                    "mllw".to_string(),
                    TestArgument {
                        argument_type: ArgumentType::Float,
                        description: "Mean lower low water elevation in NAVD 88 meters".to_string(),
                        required: true,
                    },
                ),
                (
                    "mhhw".to_string(),
                    TestArgument {
                        argument_type: ArgumentType::Float,
                        description: "Mean higher high water elevation in NAVD 88 meters"
                            .to_string(),
                        required: true,
                    },
                ),
            ]),
            test_types: vec![
                QartodTestTypes::GrossRange,
                QartodTestTypes::Spike,
                QartodTestTypes::RateOfChange,
                QartodTestTypes::FlatLine,
            ],
        }
    }

    fn scaffold(&self, arguments: HashMap<String, ArgumentValue>) -> Result<ConfigStream, String> {
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
                    threshold: 0.75 * FEET_TO_METERS,
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
        panic!("Scaffolding water level QARTOD tests for Long Island Sound with {arguments:?}",);
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
            threshold: 0.22860000000000003,
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
