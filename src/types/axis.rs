use crate::types::Axis;

impl Default for Axis {
    fn default() -> Axis {
        Axis::new()
    }
}

impl Axis {
    pub fn new() -> Axis {
        // TODO: change block centers and block edges
        Axis {
            start: 1.0,
            end: 10.0,
            step: Some(1.0),
            blocks_count: 9,
            block_centers: vec![],
            block_edges: vec![],
        }
    }
}

impl Axis {
    pub fn start(&self) -> f64 {
        self.start
    } 

    pub fn end(&self) -> f64 {
        self.end
    }

    pub fn step(&self) -> Option<f64> {
        self.step
    }

    pub fn blocks_count(&self) -> usize {
        self.blocks_count
    }

    pub fn block_centers(&self) -> &Vec<f64> {
        &self.block_centers
    }

    pub fn block_edges(&self) -> &Vec<f64> {
        &self.block_edges
    }
}
