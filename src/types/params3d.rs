use std::sync::Arc;

use crate::types::{Params3D, Axis};

impl Default for Params3D {
    fn default() -> Params3D {
        let axis_r = Arc::new(Axis::new());
        Params3D {
            axis_x: axis_r.clone(),
            axis_y: axis_r,
        }
    }
}

impl Params3D {
    pub fn new(axis_x: Arc<Axis>, axis_y: Arc<Axis>) -> Params3D {
        Params3D {
            axis_x,
            axis_y,
        }
    }
}

impl Params3D {
    pub fn axis_x(self) -> Arc<Axis> {
        self.axis_x.clone()
    }

    pub fn axis_y(self) -> Arc<Axis> {
        self.axis_y.clone()
    }
}
