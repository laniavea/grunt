pub mod types;

fn main() {
    let test_axis = types::Axis::new();
}

fn test_function() -> Result<(), Box<dyn std::error::Error>> {
    let test_axis =  types::Axis::generate_axis_edges(1.0, 2.0, None, false)?;
    Ok(())
}
