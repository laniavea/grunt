use std::sync::Arc;

#[cfg(test)]
mod tests;

pub mod types;
pub mod model3d;

fn main() {
    let res = create_model();

    match res {
        Ok(_) => (),
        Err(err) => println!("Error, {}", err),
    }
}

fn create_model() -> Result<(), Box<dyn std::error::Error>> {
    let test_axis =  Arc::new(types::Axis::generate_axis_on_edges(1.0, 2.0, None)?);

    let params = Arc::new(types::Params3D::new(test_axis.clone(), test_axis));

    let model = model3d::generate_model::generate_model3d(params);

    println!("{:?}", model);
    Ok(())
}
