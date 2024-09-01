use std::sync::Arc;

use grunt::{Params3D, Axis};

use pyo3::prelude::*;

use crate::types::{PyAxis, PyParams3D};

#[pymethods]
impl PyParams3D {
    #[new]
    fn new(axis_x: PyAxis, axis_y: PyAxis) -> PyParams3D {
        let axis_x: Arc<Axis> = Arc::new(axis_x.axis);
        let axis_y: Arc<Axis> = Arc::new(axis_y.axis);
        PyParams3D {
            params3d: Params3D::new(axis_x, axis_y),
        }
    }

    #[getter]
    fn axis_x(&self) -> PyAxis {
        let axis: Axis = (*(*self.params3d).clone().axis_x()).clone();
        PyAxis {
            axis
        }
    }

    #[getter]
    fn axis_y(&self) -> PyAxis {
        let axis: Axis = (*(*self.params3d).clone().axis_y()).clone();
        PyAxis {
            axis
        }
    }
}
