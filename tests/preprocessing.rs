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


// Robust Scaling tests

#[test]
fn robust_scaling_test_empty() {
    let vec_test: Vec<f64> = vec![];

    let result = preprocess::Preprocess::robust_scaling(vec_test.clone());
    let expected = Err("vector_numeric cannot be empty".to_string());
    assert_eq!(result, expected,"Expected {:?} but got {:?}",expected, result);
}

#[test]
fn robust_scaling_test_valid() {
    let vec_test: Vec<f64> = vec![1.0, 3.0, 2.0, 5.0];


    let expected: Vec<f64> = vec![-0.8571, 0.2857, -0.2857, 1.4286];
    let result = preprocess::Preprocess::robust_scaling(vec_test);
    
    assert_eq!(result, Ok(expected.clone()), "Expected {:?} but got {:?}",expected, result);
}



// Standard Scaling tests

#[test]
fn standard_scaling_test_empty() {
    let vec_test: Vec<f64> = vec![];

    let result = preprocess::Preprocess::standard_scaling(vec_test.clone());
    let expected = Err("vector_numeric cannot be empty".to_string());
    assert_eq!(result, expected,"Expected {:?} but got {:?}",expected, result);
}

#[test]
fn standard_scaling_test_valid() {
    let vec_test: Vec<f64> = vec![1.0, 3.0, 2.0, 5.0];

    let expected: Vec<f64> = vec![-1.1832, 0.169, -0.5071, 1.5213];
    let result = preprocess::Preprocess::standard_scaling(vec_test);
    
    assert_eq!(result, Ok(expected.clone()), "Expected {:?} but got {:?}",expected, result);
}


// Encoding Scaling tests

#[test]
fn encoder_test_empty() {
    let vec_test: Vec<String> = vec![];

    let result = preprocess::Preprocess::label_encoding(vec_test);
    let expected = Err("vector_categorical cannot be empty".to_string());
    assert_eq!(result, expected,"Expected {:?} but got {:?}",expected, result);
}

#[test]
fn encoder_test_valid() {
    let vec_test: Vec<&str> = vec!["apple", "apple", "tangerine","banana", "carrot"];
    let vec_conv = vec_test.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    
    let expected: Vec<f64> = vec![0.0, 0.0, 2.0, 3.0, 4.0];
    let result = preprocess::Preprocess::label_encoding(vec_conv);
    
    assert_eq!(result, Ok(expected.clone()), "Expected {:?} but got {:?}",expected, result);
}
