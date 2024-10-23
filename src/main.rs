use std::sync::Arc;

#[cfg(test)]
mod tests;

mod types;
mod model3d;

pub use types::{Axis, AxisError};
pub use types::{BordersParams, BorderType};
pub use types::{FillValues, FillType};

pub use types::Params3D;

pub use model3d::Model3D;
pub use model3d::generate_model3d;

fn main() {
    for i in 0..1 {
        let res = create_model(i);

        match res {
            Ok(_) => (),
            Err(err) => println!("Error, {}", err),
        }
    }

}

fn create_model(num: usize) -> Result<(), Box<dyn std::error::Error>> {
    let test_axis =  Arc::new(Axis::generate_axis_on_centers(1, 15, None)?);

    let borders_type = vec![BorderType::RandomWithStep(3, 1.0)];
    let borders_limits = vec![[35, 89], [75, 114], [95, 129]];
    let borders = Arc::new(BordersParams::new(3, &borders_type, &borders_limits)?);

    let fill_values = Arc::new(vec![FillValues::default()]);

    let params = Params3D::new(
        test_axis.clone(),
        test_axis,
        borders,
        fill_values
    );

    let model = generate_model3d(params);

    let save_state = ["params", "borders"];
    model.export_model(format!("model_3d_{num}").as_str(), &save_state).unwrap();

    // println!("{:?}", model);
    Ok(())
}
