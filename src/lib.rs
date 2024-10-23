mod types;
mod model3d;

pub use types::{Axis, AxisError};
pub use types::{BordersParams, BorderType};
pub use types::{FillValues, FillType};

pub use types::Params3D;

pub use model3d::Model3D;
pub use model3d::generate_model3d;
