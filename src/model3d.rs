use std::sync::Arc;

use crate::types::Params3D;

mod borders3d;

pub fn generate_model3d(params: Arc<Params3D>) -> Model3D {
    let _borders = borders3d::generate_borders(params.clone());
    Model3D {
        params,
    }
}

#[derive(Clone, Debug)]
pub struct Model3D {
    params: Arc<Params3D>,
}

impl Model3D {
    pub fn params(self) -> Arc<Params3D> {
        self.params.clone()
    }
}
