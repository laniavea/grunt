use super::types::Axis;

// Some static tests for axis generation to check/find float operation problems
#[test]
fn axis_generation_tests() {
    let ax = Axis::generate_axis_on_edges(1, 4, Some(1)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(*ax.blocks_centers(), vec![1.5, 2.5, 3.5]);

    let ax = Axis::generate_axis_on_edges(1.0, 4.5, None).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(*ax.blocks_centers(), vec![1.5, 2.5, 3.5]);

    let ax = Axis::generate_axis_on_edges(1.0, 5.0, Some(1.3)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 2.3, 3.6, 4.9]);
    assert_eq!(*ax.blocks_centers(), vec![1.65, 2.95, 4.25]);

    let ax = Axis::generate_axis_on_edges(1.0, 1.006, Some(0.002)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 1.002, 1.004, 1.006]);
    assert_eq!(*ax.blocks_centers(), vec![1.001, 1.003, 1.005]);

    let ax = Axis::generate_axis_on_edges(10000.0, 10000.006, Some(0.002)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![10000.0, 10000.002, 10000.004, 10000.006]);
    assert_eq!(*ax.blocks_centers(), vec![10000.001, 10000.003, 10000.005]);

    let ax = Axis::generate_axis_on_edges(-10000.0, -9999.995, Some(0.002)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![-10000.0, -9999.998, -9999.996]);
    assert_eq!(*ax.blocks_centers(), vec![-9999.999, -9999.997]);

    let ax = Axis::generate_axis_on_edges(1.0, 1.001, Some(0.001)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 1.001]);
    assert_eq!(*ax.blocks_centers(), vec![1.0005]);

    let ax = Axis::generate_axis_on_centers(1, 4, Some(1)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![0.5, 1.5, 2.5, 3.5, 4.5]);
    assert_eq!(*ax.blocks_centers(), vec![1.0, 2.0, 3.0, 4.0]);

    let ax = Axis::generate_axis_on_centers(1.0, 4.5, None).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![0.5, 1.5, 2.5, 3.5, 4.5]);
    assert_eq!(*ax.blocks_centers(), vec![1.0, 2.0, 3.0, 4.0]);

    let ax = Axis::generate_axis_on_centers(1.0, 5.0, Some(1.3)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![0.35, 1.65, 2.95, 4.25, 5.55]);
    assert_eq!(*ax.blocks_centers(), vec![1.0, 2.3, 3.6, 4.9]);

    let ax = Axis::generate_axis_on_centers(1.0, 1.004, Some(0.002)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![0.999, 1.001, 1.003, 1.005]);
    assert_eq!(*ax.blocks_centers(), vec![1.0, 1.002, 1.004]);

    let ax = Axis::generate_axis_on_centers(10000.0, 10000.005, Some(0.002)).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![9999.999, 10000.001, 10000.003, 10000.005]);
    assert_eq!(*ax.blocks_centers(), vec![10000.0, 10000.002, 10000.004]);

    //TODO: Add some big tests and check count of elems inside ax, middle element and end elements
}
