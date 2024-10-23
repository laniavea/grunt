use std::sync::Arc;

use nanoserde::SerJson;

mod axis;
pub use axis::AxisError;

mod borders;
mod fill_values;

mod params3d;

/// Struct to store Axis and some related params.
///
/// 1. Axis should ensure any elements from -10000 to 10000 with 3 decimal places.
/// 2. Axis can only be sequential.
/// 3. Start and End for axis are edges, not centers.
///
/// Axis can be devided to two parts: edges and centers. You should think about Axis as a struct to
/// store grid(image) data, so every block inside it have 2 edges and 1 center.
#[derive(Debug, Clone, SerJson)]
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
#[derive(Debug, Clone, SerJson)]
pub enum BorderType {
    /// Random value between limits
    Random,
    /// Random value between limits with step < max step(1) and probability of step(2)
    RandomWithStep(u16, f32),
}

/// Stucts to determine borders params. That means that every layer will look for it borders params
/// and be generated based on it.
/// You can determine less or more params, every layer x will be take data by module(x mod n.len)
#[derive(Debug, Clone, SerJson)]
pub struct BordersParams {
    /// Determines number of layers and borders
    number_of_borders: u8,
    /// Determines border type for every layer
    borders_type: Vec<BorderType>,
    /// Determines limits within every layer will be generated
    borders_limits: Vec<[u32; 2]>,
}

/// Enum determines method to fill values for every layer
#[derive(Debug, Clone, SerJson)]
pub enum FillType {
    /// Random value between two, both incl
    RandomBetween(i32, i32),
    /// Random value pick from vec
    ValueFrom(Vec<f32>),
}

/// Struct to store data for every layer, and some values to connect layers
#[derive(Debug, Clone, SerJson)]
pub struct FillValues {
    /// Fill type for every layer
    fill_values: Vec<FillType>,
    /// Smooth hardness (how many values can be changed to smooth)
    values_smooth: u16,
    /// Makes fill_values random or ordered
    is_preset_ordered: bool,
}

#[derive(Debug, Clone, SerJson)]
pub struct Params3D {
    axis_x: Arc<Axis>,
    axis_y: Arc<Axis>,
    borders_params: Arc<BordersParams>,
    fill_values: Arc<Vec<FillValues>>,
}
