pub mod types;

fn main() {
    let res = create_model();

    match res {
        Ok(_) => (),
        Err(err) => println!("Error, {}", err),
    }
}

fn create_model() -> Result<(), Box<dyn std::error::Error>> {
    let test_axis =  types::Axis::generate_axis_on_edges(1.0, 2.0, None)?;
    println!("{:?}", test_axis);
    Ok(())
}
