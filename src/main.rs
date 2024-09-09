use std::sync::Arc;

#[cfg(test)]
mod tests;

mod types;
mod model3d;

pub use types::Axis;
pub use types::AxisError;

pub use types::Borders;
pub use types::BorderType;

pub use types::FillValues;

pub use types::Params3D;

pub use model3d::Model3D;
pub use model3d::generate_model::generate_model3d;

fn main() {
    let res = create_model();

    match res {
        Ok(_) => (),
        Err(err) => println!("Error, {}", err),
    }

    println!("{:?}", size_of::<types::Borders>());
}

fn create_model() -> Result<(), Box<dyn std::error::Error>> {
    let test_axis =  Arc::new(Axis::generate_axis_on_edges(1, 2, None)?);

    let borders_type = vec![BorderType::Random];
    let borders_limits = vec![[19, 23]];
    let borders = Borders::new(8i32, &borders_type, &borders_limits)?;

    println!("{:?}", borders);

    let params = Params3D::new(test_axis.clone(), test_axis);

    let model = generate_model3d(params);

    println!("{:?}", model);
    Ok(())
}
