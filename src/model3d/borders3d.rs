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

    let mut borders: Vec<Vec<Vec<u32>>> = Vec::with_capacity(params.borders().number_of_borders() as usize);

    for now_border_id in 0..number_of_borders {
        let now_limits = borders_params.borders_limits()[now_border_id / number_of_borders];
        let axes_size = (ax_y_size, ax_x_size);

        borders.push(
            match border_types[now_border_id / number_of_borders] {
                BorderType::Random => {
                    random::generate_layer(axes_size, now_limits)
                },
                BorderType::RandomWithStep(step, prob) => {
                    random_with_step::generate_layer(step, prob, axes_size, now_limits)
                },
        })
    }

    borders
}
