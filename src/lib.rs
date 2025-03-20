use pyo3::prelude::*;
use seahash::hash;

/// Hash a string using seahash
#[pyfunction]
fn seahash_string(input: &str) -> u64 {
    hash(input.as_bytes())
}

/// Define the Python module
#[pymodule]
fn pyseahash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(seahash_string, m)?)?;
    Ok(())
}
