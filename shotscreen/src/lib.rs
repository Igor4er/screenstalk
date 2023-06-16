use pyo3::prelude::*;
use pyo3::types::PyBytes;
use screenshots::Screen;

#[pyfunction]
fn get_screenshots_bytes() -> PyResult<Py<PyBytes>> {
    Python::with_gil(|py| {
        let screens = Screen::all().unwrap();
        let screen = screens[0];
        let png_bytes = screen.capture().unwrap().to_png().unwrap();
        Ok(PyBytes::new(py, &png_bytes).into())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn shotscreen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_screenshots_bytes, m)?)?;
    Ok(())
}
