use pyo3::prelude::*;

mod types;

#[pymodule]
fn gruntpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<types::PyAxis>()?;
    Ok(())
}
