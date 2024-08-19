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

    let ax = Axis::generate_axis_on_edges(-10000.0, 10000.1, Some(0.2)).unwrap();
    assert_eq!((ax.blocks_edges()[50000], ax.blocks_edges()[100000]), (0.0, 10000.0));
    assert_eq!((ax.blocks_centers()[49999], ax.blocks_centers()[99999]), (-0.1, 9999.9));

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

    let ax = Axis::generate_axis_on_centers(-10000.0, 10000.1, Some(0.2)).unwrap();
    assert_eq!((ax.blocks_edges()[50001], ax.blocks_edges()[100001]), (0.1, 10000.1));
    assert_eq!((ax.blocks_centers()[50000], ax.blocks_centers()[100000]), (0.0, 10000.0));
}

// Some static tests for axis creation from vec
#[test]
fn axis_from_vec_tests() {
    let vec = vec![1, 2, 3, 4];
    let ax = Axis::from_vec_as_edges(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(*ax.blocks_centers(), vec![1.5, 2.5, 3.5]);

    let vec = vec![1.0, 2.0, 3.0, 4.5];
    let ax = Axis::from_vec_as_edges(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 2.0, 3.0, 4.5]);
    assert_eq!(*ax.blocks_centers(), vec![1.5, 2.5, 3.75]);

    let vec = vec![1.0, 1.002];
    let ax = Axis::from_vec_as_edges(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 1.002]);
    assert_eq!(*ax.blocks_centers(), vec![1.001]);

    let vec = vec![1.0, 1.00255];
    let ax = Axis::from_vec_as_edges(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 1.003]);
    assert_eq!(*ax.blocks_centers(), vec![1.002]);

    let vec = vec![1, 3, 4, 10];
    let ax = Axis::from_vec_as_edges(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 3.0, 4.0, 10.0]);
    assert_eq!(*ax.blocks_centers(), vec![2.0, 3.5, 7.0]);

    let vec = vec![1, 2];
    let ax = Axis::from_vec_as_centers(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![0.5, 1.5, 2.5]);
    assert_eq!(*ax.blocks_centers(), vec![1.0, 2.0]);

    let vec = vec![1.5f32, 2.5f32, 3.5f32];
    let ax = Axis::from_vec_as_centers(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![1.0, 2.0, 3.0, 4.0]);
    assert_eq!(*ax.blocks_centers(), vec![1.5, 2.5, 3.5]);

    let vec = vec![1.0, 1.002];
    let ax = Axis::from_vec_as_centers(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![0.999, 1.001, 1.003]);
    assert_eq!(*ax.blocks_centers(), vec![1.0, 1.002]);

    let vec = vec![1, 3, 4, 10];
    let ax = Axis::from_vec_as_centers(&vec).unwrap();
    assert_eq!(*ax.blocks_edges(), vec![0.0, 2.0, 3.5, 7.0, 13.0]);
    assert_eq!(*ax.blocks_centers(), vec![1.0, 3.0, 4.0, 10.0]);
}
