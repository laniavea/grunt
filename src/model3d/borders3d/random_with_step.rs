use rand::Rng;
use rand::distributions::{Uniform, Distribution};

pub fn generate_layer(step: u16, _prob: f32, axes_sizes: (usize, usize), limits: [u32; 2]) -> Vec<Vec<u32>> {
    let step = step as u32;

    let mut now_layer_borders: Vec<Vec<u32>> = Vec::with_capacity(axes_sizes.0);
    let mut x_axis_border: Vec<u32> = Vec::with_capacity(axes_sizes.1);

    let mut gen_rng = rand::thread_rng();
    let step_rng = Uniform::from(-(step as i32)..=step as i32);

    let mut pr_val = gen_rng.gen_range(limits[0]..=limits[1]);
    x_axis_border.push(pr_val);

    let lower_check = limits[0].saturating_add(step);
    let upper_check = limits[1].saturating_sub(step);

    for _ in 1..axes_sizes.1 {
        if pr_val < lower_check {
            pr_val = if pr_val > upper_check {
                gen_rng.gen_range(limits[0]..=limits[1])
            } else {
                gen_rng.gen_range(limits[0]..=pr_val+step)
            }
        } else if pr_val > upper_check {
            pr_val = if pr_val < lower_check {
                gen_rng.gen_range(limits[0]..=limits[1])
            } else {
                gen_rng.gen_range(pr_val-step..=limits[1])
            }
        } else {
            pr_val += step_rng.sample(&mut gen_rng) as u32;
        };

        x_axis_border.push(pr_val)
    }
    now_layer_borders.push(x_axis_border);

    now_layer_borders
}
