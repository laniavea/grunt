mod axis;

/// Struct to store Axis and some related params
/// Axis should ensure any elements from -10000 to 10000 with 3 decimal places
/// Axis can only be sequential
/// Start and End for axis are edges, not centers
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
    block_centers: Vec<f64>,
    /// Edge positions
    block_edges: Vec<f64>,
}
