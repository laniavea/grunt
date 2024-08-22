mod axis;

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
