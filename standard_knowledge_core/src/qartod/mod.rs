use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct TestSuiteInfo {
    name: String,
    slug: String,
    description: String,
    arguments: HashMap<String, TestArgument>,
    test_types: Vec<QartodTestTypes>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestArgument {
    argument_type: ArgumentType,
    description: String,
    required: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ArgumentType {
    String,
    Bool,
    Int,
    Float,
}

#[derive(Clone, Debug, PartialEq)]
pub enum QartodTestTypes {
    Location,
    GrossRange,
    Climatology,
    Spike,
    RateOfChange,
    FlatLine,
    AttenuatedSignal,
    DensityInversion,
    NearestNeighbor,
}

pub trait TestSuite: std::fmt::Debug + Send + Sync {
    fn info(&self) -> TestSuiteInfo;

    /// This should return something that and `ioos_qc.Config`
    /// https://ioos.github.io/ioos_qc/usage.html#config
    fn scaffold(&self, arguments: HashMap<String, ArgumentType>);
}

#[derive(Debug)]
struct GulfOfMaineWaterLevel {}

impl TestSuite for GulfOfMaineWaterLevel {
    fn info(&self) -> TestSuiteInfo {
        TestSuiteInfo {
            name: "Gulf of Maine".to_string(),
            slug: "gulf_of_maine".to_string(),
            description: "Water level tests for the Gulf of Maine by Hannah Baranes".to_string(),
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

    fn scaffold(&self, arguments: HashMap<String, ArgumentType>) {
        println!(
            "Scaffolding water level QARTOD tests for Gulf of Maine with {:?}",
            arguments
        )
    }
}

#[derive(Debug)]
struct LongIslandSoundWaterLevel {}

impl TestSuite for LongIslandSoundWaterLevel {
    fn info(&self) -> TestSuiteInfo {
        TestSuiteInfo {
            name: "Long Island Sound".to_string(),
            slug: "long_island_sound".to_string(),
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

    fn scaffold(&self, arguments: HashMap<String, ArgumentType>) {
        println!(
            "Scaffolding water level QARTOD tests for Long Island Sound with {:?}",
            arguments
        )
    }
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
}
