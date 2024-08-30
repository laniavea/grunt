use std::sync::Arc;

use crate::model3d::Model3D;
use crate::types::Params3D;

pub fn generate_model3d(params: Arc<Params3D>) -> Model3D {
    Model3D {
        params,
    }
}
