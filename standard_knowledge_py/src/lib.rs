pub mod standard;
pub mod standards_library;

pub use standard::PyStandard;
pub use standards_library::PyStandardsLibrary;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn _standard_knowledge_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyStandard>()?;
    m.add_class::<PyStandardsLibrary>()?;
    Ok(())
}
