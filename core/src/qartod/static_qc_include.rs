/// Static QC test suite configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticQc {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub tests: ConfigStream,
}
