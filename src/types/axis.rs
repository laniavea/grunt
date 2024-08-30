use crate::types::Axis;

// Possible errors during Axis creation
#[derive(Debug, Clone)]
pub enum AxisError {
    InvalidRange,
    TooSmallStep,
    NotOrderedVec,
    TooSmallVec,
    MinimalStep,
    NotEnoughElements,
}

impl std::fmt::Display for AxisError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AxisError::InvalidRange => writeln!(f, "End's value must be bigger than start's ones"),
            AxisError::TooSmallStep => writeln!(f, "Step value is can't be smaller than 0.001"),
            AxisError::NotOrderedVec => writeln!(f, "Values in input Vec must constanly increase"),
            AxisError::TooSmallVec => writeln!(f, "Input vector must contain at least 2 elements"),
            AxisError::MinimalStep => writeln!(f, "Minimal step for input vec must be at least 0.002"),
            AxisError::NotEnoughElements => writeln!(f, "Every generated axis must have at least two elements"),
        }
    }
}

impl std::error::Error for AxisError {}

impl Default for Axis {
    fn default() -> Axis {
        Axis::new()
    }
}

impl Axis {
    /// Creates new Axis with 9 blocks inside
    /// # Example
    ///
    /// ```
    /// use grunt::types::Axis;
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

    /// Creates Axis based on vec of input edges
    /// All digits are ROUNDED to THIRD DECIMAL places
    /// That's why input's vec minimal step must be at least 0.002
    /// # Example
    ///
    /// ```
    /// use grunt::types::Axis;
    /// let vec = vec![1, 2, 3, 4];
    /// let ax = Axis::from_vec_as_edges(&vec).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(*ax.blocks_centers(), vec![1.5, 2.5, 3.5]);
    /// ```
    ///
    /// ```
    /// use grunt::types::Axis;
    /// let vec = vec![1.0, 1.00255];
    /// let ax = Axis::from_vec_as_edges(&vec).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![1.0, 1.003]);
    /// assert_eq!(*ax.blocks_centers(), vec![1.002]);
    /// ```
    pub fn from_vec_as_edges<T: Into<f64> + Copy>(orig_edges: &[T]) -> Result<Axis, AxisError> {
        if orig_edges.len() < 2 {
            return Err(AxisError::TooSmallVec)
        }
        let mut axis_edges: Vec<f64> = Vec::with_capacity(orig_edges.len());

        let mut pr_value = f64::NEG_INFINITY;
        for edge in orig_edges {
            let now_value: f64 = ((*edge).into() * 1000.0).round() / 1000.0;
            if now_value - pr_value <= 0.001 { // If not, than edges will be also round from 0.001 to 0
                if now_value <= pr_value {
                    return Err(AxisError::NotOrderedVec);
                }
                return Err(AxisError::MinimalStep);
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

    /// Creates Axis based on vec of input centers
    /// All digits are ROUNDED to THIRD DECIMAL places
    /// That's why input's vec minimal step must be at least 0.002
    /// # Example
    ///
    /// ```
    /// use grunt::types::Axis;
    /// let vec = vec![1, 3, 4, 10];
    /// let ax = Axis::from_vec_as_centers(&vec).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.0, 2.0, 3.5, 7.0, 13.0]);
    /// assert_eq!(*ax.blocks_centers(), vec![1.0, 3.0, 4.0, 10.0]);
    /// ```
    ///
    /// ```
    /// use grunt::types::Axis;
    /// let vec = vec![1.0, 1.002];
    /// let ax = Axis::from_vec_as_centers(&vec).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.999, 1.001, 1.003]);
    /// assert_eq!(*ax.blocks_centers(), vec![1.0, 1.002]);
    /// ```
    pub fn from_vec_as_centers<T: Into<f64> + Copy>(orig_centers: &[T]) -> Result<Axis, AxisError> {
        if orig_centers.len() < 2 {
            return Err(AxisError::TooSmallVec)
        }
        let mut axis_centers: Vec<f64> = Vec::with_capacity(orig_centers.len());

        let mut pr_value = f64::NEG_INFINITY;
        for edge in orig_centers {
            let now_value: f64 = ((*edge).into() * 1000.0).round() / 1000.0;
            if now_value - pr_value <= 0.001 { // If not, than centers will be also round from 0.001 to 0
                if now_value <= pr_value {
                    return Err(AxisError::NotOrderedVec);
                }
                return Err(AxisError::MinimalStep);
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
    /// use grunt::types::Axis;
    /// let axis = Axis::generate_axis_on_edges(1, 4, Some(1)).unwrap();
    /// assert_eq!(*axis.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(*axis.blocks_centers(), vec![1.5, 2.5, 3.5]);
    /// ```
    ///
    /// ```
    /// use grunt::types::Axis;
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
    /// use grunt::types::Axis;
    /// let ax = Axis::generate_axis_on_centers(1, 4, Some(1)).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.5, 1.5, 2.5, 3.5, 4.5]);
    /// assert_eq!(*ax.blocks_centers(), vec![1.0, 2.0, 3.0, 4.0]);
    /// ```
    ///
    /// ```
    /// use grunt::types::Axis;
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

        if iter_count < 2 {
            return Err(AxisError::NotEnoughElements)
        }

        Ok((start, step, iter_count))
    }

    // Function to generate centers based on block's edges, required more than 2 values inside vec
    fn centers_from_edges(axis_edges: &[f64], step: Option<f64>) -> Vec<f64> {
        let mut axis_centers: Vec<f64> = Vec::with_capacity(axis_edges.len() - 1);

        if let Some(step) = step {
            let mut pr_value = axis_edges[0] + step / 2.0;
            for _ in 0..(axis_edges.len() - 1) {
                axis_centers.push(pr_value);
                pr_value = ((pr_value + step) * 1000.0).round() / 1000.0;
            }
        } else {
            let mut pr_value = axis_edges[0];
            for now_elem in &axis_edges[1..] {
                axis_centers.push(((pr_value + now_elem) * 500.0).round() / 1000.0);
                pr_value = *now_elem;
            }
        }

        axis_centers
    }

    // Function to generate edges based on block's centers, required more than 2 values inside vec
    fn edges_from_centers(axis_centers: &[f64], step: Option<f64>) -> Vec<f64> {
        let mut axis_edges: Vec<f64> = Vec::with_capacity(axis_centers.len() + 1);

        if let Some(step) = step {
            let mut pr_value = axis_centers[0] - step / 2.0;
            for _ in 0..(axis_centers.len() + 1) {
                axis_edges.push(pr_value);
                pr_value = ((pr_value + step) * 1000.0).round() / 1000.0;
            }
        } else {
            let mut pr_value = axis_centers[0];
            let mut last_diff = 0.0;

            axis_edges.push(((pr_value - (axis_centers[1] - pr_value) / 2.0) * 1000.0).round() / 1000.0);
            for now_elem in &axis_centers[1..] {
                last_diff = (now_elem - pr_value) / 2.0;
                axis_edges.push(((pr_value + last_diff) * 1000.0).round() / 1000.0);
                pr_value = *now_elem;
            }
            axis_edges.push(((pr_value + last_diff) * 1000.0).round() / 1000.0);
        }

        axis_edges
    }
}

impl Axis {
    /// Returns first edge for axis
    /// # Example
    /// ```
    /// use grunt::types::Axis;
    /// let ax = Axis::generate_axis_on_centers(1.0, 2.0, Some(0.5)).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.75, 1.25, 1.75, 2.25]);
    /// assert_eq!(ax.start(), 0.75);
    /// ```
    pub fn start(&self) -> f64 {
        self.start
    } 

    /// Returns last edge for axis
    /// # Example
    /// ```
    /// use grunt::types::Axis;
    /// let ax = Axis::generate_axis_on_centers(1.0, 2.0, Some(0.5)).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.75, 1.25, 1.75, 2.25]);
    /// assert_eq!(ax.end(), 2.25);
    /// ```
    pub fn end(&self) -> f64 {
        self.end
    }

    /// Returns step if axis was generated with it, otherwise - None
    /// # Example
    /// ```
    /// use grunt::types::Axis;
    /// let ax = Axis::generate_axis_on_centers(1.0, 2.0, Some(0.5)).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.75, 1.25, 1.75, 2.25]);
    /// assert_eq!(ax.step(), Some(0.5));
    /// ```
    pub fn step(&self) -> Option<f64> {
        self.step
    }

    /// Returns number of blocks inside axis, block - "object" that is between two edges 
    /// # Example
    /// ```
    /// use grunt::types::Axis;
    /// let ax = Axis::generate_axis_on_edges(1.0, 2.0, Some(0.5)).unwrap();
    /// assert_eq!(ax.blocks_count(), 2);
    /// ```
    pub fn blocks_count(&self) -> usize {
        self.blocks_count
    }

    /// Returns blocks centers, where block is "object" that is between two edges
    /// # Example
    /// ```
    /// use grunt::types::Axis;
    /// let ax = Axis::generate_axis_on_edges(1.0, 2.0, Some(0.5)).unwrap();
    /// assert_eq!(*ax.blocks_centers(), vec![1.25, 1.75]);
    /// ```
    pub fn blocks_centers(&self) -> &Vec<f64> {
        &self.blocks_centers
    }

    /// Returns axises edges, edges represents borders, which are separating two block
    /// # Example
    /// ```
    /// use grunt::types::Axis;
    /// let ax = Axis::generate_axis_on_centers(1.0, 2.0, Some(0.5)).unwrap();
    /// assert_eq!(*ax.blocks_edges(), vec![0.75, 1.25, 1.75, 2.25]);
    /// ```
    pub fn blocks_edges(&self) -> &Vec<f64> {
        &self.blocks_edges
    }
}
