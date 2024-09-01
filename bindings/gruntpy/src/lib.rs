use pyo3::prelude::*;

mod types;

#[pymodule]
fn gruntpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<types::PyAxis>()?;
    m.add_class::<types::PyParams3D>()?;
    Ok(())
}
