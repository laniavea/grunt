use std::sync::Arc;

use crate::types::{BorderType, Params3D};

mod random;
mod random_with_step;

/// Function to generate borders data for model
/// Borders are represented as border_num -> y -> x
pub fn generate_borders(params: Arc<Params3D>) -> Vec<Vec<Vec<u32>>> {
    let ax_y_size = params.axis_y().blocks_centers().len();
    let ax_x_size = params.axis_x().blocks_centers().len();

    let borders_params = params.borders();
    let number_of_borders = borders_params.number_of_borders() as usize;
    let border_types = borders_params.borders_type();

    let now_limits_count = borders_params.borders_limits().len();
    let now_border_types_count = border_types.len();

    let mut borders: Vec<Vec<Vec<u32>>> = Vec::with_capacity(params.borders().number_of_borders() as usize);

    for now_border_id in 0..number_of_borders {
        let now_limits = borders_params.borders_limits()[now_border_id % now_limits_count];
        println!("{:?}", now_limits);
        let axes_size = (ax_y_size, ax_x_size);

        borders.push(
            match border_types[now_border_id % now_border_types_count] {
                BorderType::Random => {
                    random::generate_layer(axes_size, now_limits)
                },
                BorderType::RandomWithStep(step, prob) => {
                    random_with_step::generate_layer(step, prob, axes_size, now_limits)
                },
        });

        //TODO: If validation needed
        match border_types[now_border_id / number_of_borders] {
            BorderType::Random => { validate_layer(&borders[borders.len() - 1], now_limits, None)},
            BorderType::RandomWithStep(step, _prob) => {
                validate_layer(&borders[borders.len() - 1], now_limits, Some(step));
            }
        }
    }

    borders
}

//TODO: Rewrite to Result type and logging
pub fn validate_layer(border_to_check: &[Vec<u32>], limits: [u32; 2], step: Option<u16>) {
    // for i in border_to_check {
    //     for j in i {
    //         print!("{j}\t")
    //     }
    //     println!();
    // }

    if border_to_check.len() < 2 || border_to_check[0].len() < 2 {
        panic!("Cannot validate layer because of small size");
    }

    let mut pr_el = border_to_check[0][0];
    if pr_el < limits[0] || pr_el > limits[1] {
        panic!("Element y - 0; x - 0: out of limits bounds");
    }

    for (now_id, now_el) in border_to_check[0].iter().enumerate().skip(1) {
        if *now_el < limits[0] || *now_el > limits[1] {
            panic!("Element y - 0; x - {now_id}: out of limits bounds");
        }

        if step.is_some() && pr_el.abs_diff(*now_el) as u16 > step.unwrap() {
            panic!("Element y - 0; x - {now_id}: step overflow");
        }

        pr_el = *now_el;
    }

    for (now_y_id, now_y) in border_to_check.iter().enumerate().skip(1) {
        let mut pr_val = now_y[0];
        if pr_val < limits[0] || pr_val > limits[1] {
            panic!("Element y - {now_y_id}; x - 0: out of limits bounds");
        }

        for (now_x_id, now_x) in now_y.iter().enumerate().skip(1) {
            if *now_x < limits[0] || *now_x > limits[1] {
                panic!("Element y - {now_y_id}; x - {now_x_id}: out of limits bounds");
            }

            if step.is_some() && pr_val.abs_diff(*now_x) as u16 > step.unwrap() {
                panic!("Element y - {now_y_id}; x - {now_x_id}: step overflow PREVIOUS");
            }

            if step.is_some() && border_to_check[now_y_id-1][now_x_id].abs_diff(*now_x) as u16 > step.unwrap() {
                panic!("Element y - {now_y_id}; x - {now_x_id}: step overflow UPPER");
            }

            pr_val = *now_x;
        }
    }
}
