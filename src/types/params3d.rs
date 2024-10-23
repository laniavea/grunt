use std::sync::Arc;

use crate::types::{Params3D, Axis, BordersParams, FillValues};

impl Default for Params3D {
    fn default() -> Params3D {
        let axis_r = Arc::new(Axis::new());
        let borders = Arc::new(BordersParams::default());
        let fill_values = Arc::new(vec![FillValues::default()]);
        Params3D {
            axis_x: axis_r.clone(),
            axis_y: axis_r,
            borders_params: borders,
            fill_values,
        }
    }
}

impl Params3D {
    pub fn new(
        axis_x: Arc<Axis>,
        axis_y: Arc<Axis>,
        borders: Arc<BordersParams>,
        fill_values: Arc<Vec<FillValues>>
    ) -> Arc<Params3D> {

        Arc::new(Params3D {
            axis_x,
            axis_y,
            borders_params: borders,
            fill_values
        })
    }
}

impl Params3D {
    pub fn axis_x(&self) -> Arc<Axis> {
        self.axis_x.clone()
    }

    pub fn axis_y(&self) -> Arc<Axis> {
        self.axis_y.clone()
    }

    pub fn borders(&self) -> Arc<BordersParams> {
        self.borders_params.clone()
    }

    pub fn fill_values(&self) -> Arc<Vec<FillValues>> {
        self.fill_values.clone()
    }
}
