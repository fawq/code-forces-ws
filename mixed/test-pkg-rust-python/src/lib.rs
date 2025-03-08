use pyo3::prelude::*;

#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from test-pkg-rust-python!".to_string()
}

#[pyfunction]
fn get_fine_number() -> u32 {
    55
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(get_fine_number, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert() {
        let result = get_fine_number() + 2;
        assert_eq!(result, 57);
    }
}
