mod types;
mod model3d;

pub use types::Axis;
pub use types::axis::AxisError;

pub use types::Borders;
pub use types::BorderType;

pub use types::Params3D;

pub use model3d::Model3D;
pub use model3d::generate_model::generate_model3d;
