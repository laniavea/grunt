use grunt::Axis;
use crate::types::PyAxis;

use pyo3::prelude::*;

#[pymethods]
impl PyAxis {
    #[new]
    fn new() -> PyAxis {
        PyAxis {
            axis: Axis::new()
        }
    }

    #[getter]
    fn start(&self) -> f64 {
        self.axis.start()
    }

    #[getter]
    fn end(&self) -> f64 {
        self.axis.end()
    }

    #[getter]
    fn step(&self) -> Option<f64> {
        self.axis.step()
    }

    #[getter]
    fn blocks_count(&self) -> usize {
        self.axis.blocks_count()
    }

    #[getter]
    fn blocks_centers(&self) -> Vec<f64> {
        self.axis.blocks_centers().to_vec()
    }

    #[getter]
    fn blocks_edges(&self) -> Vec<f64> {
        self.axis.blocks_edges().to_vec()
    }
}
