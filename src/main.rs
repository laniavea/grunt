use std::sync::Arc;

#[cfg(test)]
mod tests;

mod types;
mod model3d;

pub use types::{Axis, AxisError};
pub use types::{Borders, BorderType};
pub use types::{FillValues, FillType};

pub use types::Params3D;

pub use model3d::Model3D;
pub use model3d::generate_model3d;

fn main() {
    let res = create_model();

    match res {
        Ok(_) => (),
        Err(err) => println!("Error, {}", err),
    }
}

fn create_model() -> Result<(), Box<dyn std::error::Error>> {
    let test_axis =  Arc::new(Axis::generate_axis_on_edges(1, 1000, None)?);

    let borders_type = vec![BorderType::Random];
    let borders_limits = vec![[19, 23]];
    let borders = Arc::new(Borders::new(1, &borders_type, &borders_limits)?);

    let fill_values = Arc::new(vec![FillValues::default()]);

    let params = Params3D::new(
        test_axis.clone(),
        test_axis,
        borders,
        fill_values
    );

    let model = generate_model3d(params);

    println!("{:?}", model);
    Ok(())
}
