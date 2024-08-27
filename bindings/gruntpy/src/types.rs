use grunt::Axis;

use pyo3::prelude::*;

mod axis;

#[pyclass]
pub struct PyAxis{
    axis: Axis
}
