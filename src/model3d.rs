use std::sync::Arc;

use crate::types::Params3D;

pub mod generate_model;

#[derive(Clone, Debug)]
pub struct Model3D {
    params: Arc<Params3D>,
}

impl Model3D {
    pub fn params(self) -> Arc<Params3D> {
        self.params.clone()
    }
}
