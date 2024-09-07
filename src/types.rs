use std::sync::Arc;

pub mod axis;
pub mod params3d;
pub mod borders;

/// Struct to store Axis and some related params.
///
/// 1. Axis should ensure any elements from -10000 to 10000 with 3 decimal places.
/// 2. Axis can only be sequential.
/// 3. Start and End for axis are edges, not centers.
///
/// Axis can be devided to two parts: edges and centers. You should think about Axis as a struct to
/// store grid(image) data, so every block inside it have 2 edges and 1 center.
#[derive(Debug, Clone)]
pub struct Axis {
    /// First edge for axis
    start: f64,
    /// Last edge for axis
    end: f64,
    /// Step between edges if exists
    step: Option<f64>,
    /// Number of centers(blocks)
    blocks_count: usize,
    /// Center positions
    blocks_centers: Vec<f64>,
    /// Edge positions
    blocks_edges: Vec<f64>,
}

/// Enum determines method to generate borders
#[derive(Debug, Clone)]
pub enum BorderType {
    /// Random value between limits
    Random,
    /// Random value between limits with step < max step(1) and probability of step(2)
    RandomWithStep(u16, f32),
}

/// Stucts to determine borders params. That means that every layer will look for it borders params
/// and be generated based on it.
/// You can determine less or more params, every layer x will be take data by module(x mod n.len)
#[derive(Debug, Clone)]
pub struct Borders {
    /// Determines number of layers and borders
    number_of_borders: u8,
    /// Determines border type for every layer
    borders_type: Vec<BorderType>,
    /// Determines limits within every layer will be generated
    borders_limits: Vec<[u32; 2]>,
}

#[derive(Debug, Clone)]
pub struct Params3D {
    axis_x: Arc<Axis>,
    axis_y: Arc<Axis>,
}
