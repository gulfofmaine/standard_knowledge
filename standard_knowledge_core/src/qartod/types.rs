use std::collections::HashMap;
use std::fmt::Display;

use dyn_clone::DynClone;

// Re-export config types for convenience
use super::config::{ArgumentValue, Config};

#[derive(Clone, Debug, PartialEq)]
pub struct TestSuiteInfo {
    pub name: String,
    pub slug: String,
    pub summary: String,
    pub description: String,
    pub arguments: HashMap<String, TestArgument>,
    pub test_types: Vec<QartodTestTypes>,
}

impl Display for TestSuiteInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}): {}", self.name, self.slug, self.summary)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestArgument {
    pub argument_type: ArgumentType,
    pub description: String,
    pub required: bool,
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

pub trait TestSuite: std::fmt::Debug + Send + Sync + DynClone {
    fn info(&self) -> TestSuiteInfo;

    /// This should return a Config that represents an `ioos_qc.Config`
    /// https://ioos.github.io/ioos_qc/usage.html#config
    fn scaffold(&self, arguments: HashMap<String, ArgumentValue>) -> Config;
}

dyn_clone::clone_trait_object!(TestSuite);
