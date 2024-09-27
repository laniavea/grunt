use rand::distributions::{Uniform, Distribution};

/// Funtion to generate layer based on limits and nothing more.
/// axes_sizes - two usize, first - Y axis, second - X axis.
/// limits - [min_bound, max_bound]
pub fn generate_layer(axes_sizes: (usize, usize), limits: [u32; 2]) -> Vec<Vec<u32>> {
    let mut now_layer_borders: Vec<Vec<u32>> = Vec::with_capacity(axes_sizes.0);

    let mut rng = rand::thread_rng();
    let gen_range = Uniform::from(limits[0]..=limits[1]);

    for _ in 0..axes_sizes.0 {
        let mut x_axis_border: Vec<u32> = Vec::with_capacity(axes_sizes.1);

        for _ in 0..axes_sizes.1 {
            x_axis_border.push(gen_range.sample(&mut rng));
        }
        now_layer_borders.push(x_axis_border);
    }

    now_layer_borders
}
