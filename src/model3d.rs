use std::sync::Arc;

use nanoserde::SerJson;

use crate::types::Params3D;

mod borders3d;
pub mod export;

pub fn generate_model3d(params: Arc<Params3D>) -> Model3D {
    let borders = borders3d::generate_borders(params.clone());
    Model3D {
        params,
        borders,
    }
}

#[derive(Clone, Debug, SerJson)]
pub struct Model3D {
    params: Arc<Params3D>,
    borders: Vec<Vec<Vec<u32>>>,
}

impl Model3D {
    pub fn params(&self) -> Arc<Params3D> {
        self.params.clone()
    }

    pub fn borders(&self) -> &Vec<Vec<Vec<u32>>> {
        &self.borders
    }
}
