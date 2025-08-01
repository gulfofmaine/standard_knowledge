use std::collections::HashMap;

// // Re-export config types for convenience
// pub use config::{ArgumentValue, Call, Config, ConfigBuilder, Context, ContextConfig, TimeWindow};

pub mod config;
pub mod static_qc;
pub mod types;
pub mod water_level;

pub use static_qc::StaticQcTestSuite;
pub use types::TestSuite;

pub fn test_suites() -> HashMap<String, Vec<Box<dyn TestSuite>>> {
    let mut suites: HashMap<String, Vec<Box<dyn TestSuite>>> = HashMap::new();

    // Add water level test suites
    suites.extend(water_level::water_level_test_suites());

    suites
}
