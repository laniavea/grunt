use std::sync::Arc;

use grunt::{Axis, Params3D};

use pyo3::prelude::*;

mod axis;
mod params3d;

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyAxis{
    axis: Axis,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyParams3D {
    params3d: Arc<Params3D>,
}
