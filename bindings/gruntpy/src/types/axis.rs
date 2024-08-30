use grunt::Axis;

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::types::PyAxis;

#[pymethods]
impl PyAxis {
    #[new]
    fn new() -> PyAxis {
        PyAxis {
            axis: Axis::new()
        }
    }

    #[staticmethod]
    fn from_vec_as_edges(input_nums: Vec<f64>) -> PyResult<PyAxis> {
        match Axis::from_vec_as_edges(&input_nums) {
            Ok(axis) => Ok(PyAxis {
                axis
            }),
            Err(e) => {
                Err(PyValueError::new_err(format!("Axis error: {e}")))
            }
        }
    }

    #[staticmethod]
    fn from_vec_as_centers(input_nums: Vec<f64>) -> PyResult<PyAxis> {
        match Axis::from_vec_as_centers(&input_nums) {
            Ok(axis) => Ok(PyAxis {
                axis
            }),
            Err(e) => {
                Err(PyValueError::new_err(format!("Axis error: {e}")))
            }
        }
    }

    #[staticmethod]
    #[pyo3(signature = (start, end, step=None))]
    fn generate_axis_on_edges(start: f64, end: f64, step: Option<f64>) -> PyResult<PyAxis> {
        match Axis::generate_axis_on_edges(start, end, step) {
            Ok(axis) => Ok(PyAxis {
                axis
            }),
            Err(e) => {
                Err(PyValueError::new_err(format!("Axis error: {e}")))
            }
        }
    }

    #[staticmethod]
    #[pyo3(signature = (start, end, step=None))]
    fn generate_axis_on_centers(start: f64, end: f64, step: Option<f64>) -> PyResult<PyAxis> {
        match Axis::generate_axis_on_centers(start, end, step) {
            Ok(axis) => Ok(PyAxis {
                axis
            }),
            Err(e) => {
                Err(PyValueError::new_err(format!("Axis error: {e}")))
            }
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
