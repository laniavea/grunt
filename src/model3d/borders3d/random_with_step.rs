use rand::Rng;
use rand::distributions::{Uniform, Distribution};

pub fn generate_layer(step: u16, _prob: f32, axes_sizes: (usize, usize), limits: [u32; 2]) -> Vec<Vec<u32>> {
    let step = step as u32;

    // Bounds when it is reasonable to check possible limites overflow
    let lower_check = limits[0].saturating_add(step);
    let upper_check = limits[1].saturating_sub(step);

    let mut gen_rng = rand::thread_rng();
    let limits_rng = Uniform::from(limits[0]..=limits[1]);
    let step_rng = Uniform::from(-(step as i32)..=step as i32);

    let mut now_layer_borders: Vec<Vec<u32>> = Vec::with_capacity(axes_sizes.0);
    let mut x_axis_border: Vec<u32> = Vec::with_capacity(axes_sizes.1);

    let mut pr_val = limits_rng.sample(&mut gen_rng);
    x_axis_border.push(pr_val);

    // Loop to fill only first row (y=0)
    for _ in 1..axes_sizes.1 {
        if pr_val < lower_check {
            pr_val = if pr_val > upper_check {
                limits_rng.sample(&mut gen_rng)
            } else {
                gen_rng.gen_range(limits[0]..=pr_val+step)
            }
        } else if pr_val > upper_check {
            pr_val = if pr_val < lower_check {
                limits_rng.sample(&mut gen_rng)
            } else {
                gen_rng.gen_range(pr_val-step..=limits[1])
            }
        } else {
            let now_step = step_rng.sample(&mut gen_rng);
            if now_step > 0 {
                pr_val += now_step as u32;
            } else {
                pr_val -= now_step.unsigned_abs();
            }
        };

        x_axis_border.push(pr_val);
    }
    now_layer_borders.push(x_axis_border);

    // Loop to fill every y from 1 to y size
    for now_y in 1..axes_sizes.0 {
        let mut x_axis_border: Vec<u32> = Vec::with_capacity(axes_sizes.1);
        let pr_x_ax = &now_layer_borders[now_y - 1];
        let mut pr_val = pr_x_ax[0];

        if pr_val < lower_check {
            pr_val = if pr_val > upper_check {
                limits_rng.sample(&mut gen_rng)
            } else {
                gen_rng.gen_range(limits[0]..=pr_val+step)
            }
        } else if pr_val > upper_check {
            pr_val = if pr_val < lower_check {
                limits_rng.sample(&mut gen_rng)
            } else {
                gen_rng.gen_range(pr_val-step..=limits[1])
            }
        } else {
            let now_step = step_rng.sample(&mut gen_rng);
            if now_step > 0 {
                pr_val += now_step as u32;
            } else {
                pr_val -= now_step.unsigned_abs();
            }
        };
        x_axis_border.push(pr_val);

        for now_x in 1..axes_sizes.1 {
            let upper_value = pr_x_ax[now_x];

            if upper_value >= pr_val {
                let now_up_limit = if pr_val > upper_check {
                    limits[1]
                } else {
                    pr_val + step
                };

                let now_down_limit = if upper_value < lower_check {
                    limits[0]
                } else {
                    upper_value - step
                };

                pr_val = gen_rng.gen_range(now_down_limit..=now_up_limit);
            } else {
                let now_up_limit = if upper_value > upper_check {
                    limits[1]
                } else {
                    upper_value + step
                };

                let now_down_limit = if pr_val < lower_check {
                    limits[0]
                } else {
                    pr_val - step
                };

                pr_val = gen_rng.gen_range(now_down_limit..=now_up_limit);
            }
            
            x_axis_border.push(pr_val);
        }

        now_layer_borders.push(x_axis_border);
    }

    now_layer_borders
}
