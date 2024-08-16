use machine_learning::algos::preprocess::{self, *};

// Log Scaling tests
#[test]
fn log_scaling_test() {
    let vec_test: Vec<f64> = vec![1.0, 3.0, 2.0, 5.0];


    let vec_result: Vec<f64> = vec![0.0, 1.09861, 0.69315, 1.60944];

    assert_eq!(preprocess::Preprocess::log_scaling(vec_test.clone()), Ok(vec_result.clone()),
    "Expected {:?} but got {:?}",vec_result, preprocess::Preprocess::log_scaling(vec_test));
}

#[test]
fn log_scaling_test_empty() {
    let vec_test: Vec<f64> = vec![];

    let result = Err("vector_numeric cannot be empty".to_string());

    assert_eq!(preprocess::Preprocess::log_scaling(vec_test.clone()), result,
    "Expected {:?} but got {:?}",result, preprocess::Preprocess::log_scaling(vec_test));
}