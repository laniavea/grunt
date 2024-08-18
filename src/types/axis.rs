use crate::types::Axis;

#[derive(Debug, Clone)]
pub enum AxisError {
    InvalidRange,
    TooSmallStep,
    NotOrderedVec,
}

impl std::fmt::Display for AxisError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AxisError::InvalidRange => writeln!(f, "End's value must be bigger than start's ones"),
            AxisError::TooSmallStep => writeln!(f, "Step value is can't be smaller than 0.001"),
            AxisError::NotOrderedVec => writeln!(f, "Values in input Vec must constanly increase"),
        }
    }
}

impl std::error::Error for AxisError {}

impl Default for Axis {
    fn default() -> Axis {
        Axis::new()
    }
}

//TODO: std::fmt::Display for axis?

impl Axis {
    /// Creates new Axis with 9 blocks inside
    /// # Example
    ///
    /// ```
    /// let axis = Axis::new();
    /// assert_eq!((axis.start(), axis.end(), axis.step()), (1.0, 10.0, Some(1.0)));
    /// assert_eq!(*axis.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    /// assert_eq!(*axis.blocks_centers(), vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5]);
    pub fn new() -> Axis {
        Axis {
            start: 1.0,
            end: 10.0,
            step: Some(1.0),
            blocks_count: 9,
            blocks_centers: vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5, 9.5],
            blocks_edges: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        }
    }

    pub fn from_vec_as_edges<T: Into<f64> + Copy>(orig_edges: &[T]) -> Result<Axis, AxisError> {
        let mut axis_edges: Vec<f64> = Vec::with_capacity(orig_edges.len());

        let mut pr_value = f64::NEG_INFINITY;
        for edge in orig_edges {
            let now_value: f64 = (*edge).into();
            if now_value <= pr_value {
                return Err(AxisError::NotOrderedVec);
            }
            axis_edges.push(now_value);
            pr_value = now_value;
        }

        let axis_centers = Axis::centers_from_edges(&axis_edges, None);

        Ok(Axis {
            start: axis_edges[0],
            end: axis_edges[axis_edges.len() - 1],
            step: None,
            blocks_count: axis_centers.len(),
            blocks_centers: axis_centers,
            blocks_edges: axis_edges,
        })
    }

    pub fn from_vec_as_centers<T: Into<f64> + Copy>(orig_centers: &[T]) -> Result<Axis, AxisError> {
        let mut axis_centers: Vec<f64> = Vec::with_capacity(orig_centers.len());

        let mut pr_value = f64::NEG_INFINITY;
        for edge in orig_centers {
            let now_value: f64 = (*edge).into();
            if now_value <= pr_value {
                return Err(AxisError::NotOrderedVec);
            }
            axis_centers.push(now_value);
            pr_value = now_value;
        }

        let axis_edges = Axis::edges_from_centers(&axis_centers, None);

        Ok(Axis {
            start: axis_edges[0],
            end: axis_edges[axis_edges.len() - 1],
            step: None,
            blocks_count: axis_centers.len(),
            blocks_centers: axis_centers,
            blocks_edges: axis_edges,
        })
    }

    /// Function to create axis based on generated edges using start, end and step
    /// If step is None it will be replaced to 1.0, step must be always bigger than 0.001
    /// Start must be always smaller than end
    /// Axis recommended to be between -10000 and 10000, because possible float related errors
    /// # Examples
    /// ```
    /// let axis = Axis::generate_axis_on_edges(1, 4, Some(1)).unwrap();
    /// assert_eq!(*axis.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(*axis.blocks_centers(), vec![1.5, 2.5, 3.5]);
    /// ```
    ///
    /// ```
    /// let ax = Axis::generate_axis_on_edges(1.0, 4.5, None).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(*ax.blocks_centers(), vec![1.5, 2.5, 3.5]);
    /// ```
    pub fn generate_axis_on_edges<T: Into<f64>>(start: T, end: T, step: Option<T>) -> Result<Axis, AxisError> {
        let (start, step, iter_count) = Axis::get_generation_info(start, end, step)?;

        let mut axis_edges: Vec<f64> = Vec::with_capacity(iter_count);

        let mut pr_value = start;
        for _ in 0..iter_count {
            axis_edges.push(pr_value);
            pr_value = ((pr_value + step) * 1000.0).round() / 1000.0;
        }

        let axis_centers = Axis::centers_from_edges(&axis_edges, Some(step));

        Ok(Axis {
            start,
            end: axis_edges[axis_edges.len() - 1],
            step: Some(step),
            blocks_count: axis_centers.len(),
            blocks_centers: axis_centers,
            blocks_edges: axis_edges,
        })
    }

    /// Function to create axis based on generated block's centers using start, end and step
    /// If step is None it will be replaced to 1.0, step must be always bigger than 0.001
    /// Start must be always smaller than end
    /// Axis recommended to be between -10000 and 10000, because possible float related errors
    /// # Examples
    /// ```
    /// let ax = Axis::generate_axis_on_centers(1, 4, Some(1)).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.5, 1.5, 2.5, 3.5, 4.5]);
    /// assert_eq!(*ax.blocks_centers(), vec![1.0, 2.0, 3.0, 4.0]);
    /// ```
    ///
    /// ```
    /// let ax = Axis::generate_axis_on_centers(1.0, 4.5, None).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.5, 1.5, 2.5, 3.5, 4.5]);
    /// assert_eq!(*ax.blocks_centers(), vec![1.0, 2.0, 3.0, 4.0]);
    /// ```
    pub fn generate_axis_on_centers<T: Into<f64>>(start: T, end: T, step: Option<T>) -> Result<Axis, AxisError> {
        let (start, step, iter_count) = Axis::get_generation_info(start, end, step)?;

        let mut axis_centers: Vec<f64> = Vec::with_capacity(iter_count);

        let mut pr_value = start;
        for _ in 0..iter_count {
            axis_centers.push(pr_value);
            pr_value = ((pr_value + step) * 1000.0).round() / 1000.0;
        }

        let axis_edges = Axis::edges_from_centers(&axis_centers, Some(step));

        Ok(Axis {
            start: axis_edges[0],
            end: axis_edges[axis_edges.len() - 1],
            step: Some(step),
            blocks_count: axis_centers.len(),
            blocks_centers: axis_centers,
            blocks_edges: axis_edges,
        })
    }
}

impl Axis {
    // Function to convert input data for axis generation to f64/usize.
    fn get_generation_info<T: Into<f64>>(start: T, end: T, step: Option<T>) -> Result<(f64, f64, usize), AxisError> {
        let start: f64 = (start.into() * 1000.0).round() / 1000.0;
        let end: f64 = (end.into() * 1000.0).round() / 1000.0;

        if start >= end {
            return Err(AxisError::InvalidRange)
        }

        let step: f64 = if let Some(step) = step {
            let temp_step = (step.into() * 1000.0).round() / 1000.0;
            if temp_step >= 0.001 {
                temp_step
            } else {
                return Err(AxisError::TooSmallStep);
            }
        } else {
            1.0
        };

        let iter_count = (((end-start) * 1000.0).round() / 1000.0 / step).floor() as usize + 1;
        Ok((start, step, iter_count))
    }
    fn centers_from_edges(axis_edges: &[f64], step: Option<f64>) -> Vec<f64> {
        let mut axis_centers: Vec<f64> = Vec::with_capacity(axis_edges.len() - 1);

        if let Some(step) = step {
            let mut pr_value = axis_edges[0] + step / 2.0;
            for _ in 0..(axis_edges.len() - 1) {
                axis_centers.push(pr_value);
                pr_value = ((pr_value + step) * 1000.0).round() / 1000.0;
            }
        } else {
            //TODO: Reanalyze this
            let mut pr_value = axis_edges[0];
            for now_elem in &axis_edges[1..] {
                axis_centers.push(((pr_value + now_elem) * 500.0).round() / 1000.0);
                pr_value = *now_elem;
            }
        }

        axis_centers
    }

    fn edges_from_centers(axis_centers: &[f64], step: Option<f64>) -> Vec<f64> {
        let mut axis_edges: Vec<f64> = Vec::with_capacity(axis_centers.len() + 1);

        if let Some(step) = step {
            let mut pr_value = axis_centers[0] - step / 2.0;
            for _ in 0..(axis_centers.len() + 1) {
                axis_edges.push(pr_value);
                pr_value = ((pr_value + step) * 1000.0).round() / 1000.0;
            }
        } else {
            //TODO: Check this
            let mut pr_value = axis_edges[0];
            for now_elem in &axis_centers[1..] {
                axis_edges.push(((pr_value - (now_elem - pr_value)) * 1000.0).round() / 1000.0);
                pr_value = *now_elem;
            }
        }

        axis_edges
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

    pub fn blocks_centers(&self) -> &Vec<f64> {
        &self.blocks_centers
    }

    pub fn blocks_edges(&self) -> &Vec<f64> {
        &self.blocks_edges
    }
}
