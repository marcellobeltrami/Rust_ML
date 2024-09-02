use machine_learning::algos::array_ops::*;

// Matrix sum tests
#[test]
fn mx_sum() {
    let mx_1: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![2.0, 5.0]];

    let mx1 = Matrix { main_matrix: mx_1 };
    let mx_2: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![2.0, 5.0]];

    assert_eq!(
        mx1.mx_sum(mx_2),
        Ok(vec![vec![2.0, 6.0], vec![4.0, 10.0]])
    );
}

#[test]
fn mx_sum_uneven() {
    let mx_1: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![2.0, 5.0]];

    let mx1 = Matrix { main_matrix: mx_1 };

    let mx_2: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0, 5.0]];

    let result = mx1.mx_sum(mx_2);

    assert_eq!(result, Err("Matrices must have the same row lengths".to_string()), "Expected Matrices must have the same row lengths but got {:?}", result);
}

// Matrix subtraction tests
#[test]
fn mx_subtract_valid() {
    let mx_1: Vec<Vec<f64>> = vec![
        vec![6.0, 5.0], 
        vec![20.0, 34.0]
    ];

    let mx1 = Matrix { main_matrix: mx_1 };

    let mx_2: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0], 
        vec![3.0, 4.0]
    ];

    let expected_result: Vec<Vec<f64>> = vec![
        vec![5.0, 3.0], 
        vec![17.0, 30.0]
    ];

    let result = mx1.mx_subtract(mx_2);

    assert_eq!(result, Ok(expected_result.clone()), "Expected {:?}, but got {:?}", expected_result, result);
}

// Matrix subtraction tests
#[test]
fn mx_subtract_invalid() {
    let mx_1: Vec<Vec<f64>> = vec![
        vec![6.0, 5.0], 
        vec![20.0, 34.0]
    ];

    let mx1 = Matrix { main_matrix: mx_1 };

    let mx_2: Vec<Vec<f64>> = vec![
        vec![5.0, 3.0], 
        vec![17.0, 30.0],
        vec![17.0, 30.0]
    ];

    let result = mx1.mx_subtract(mx_2);

    assert_eq!(result, Err("Matrices must have the same dimensions".to_string()), "Expected 'Err(Matrices must have the same dimensions)', but got {:?}", result);
}

// Matrix multiplication tests
#[test]
fn mx_mult_valid() {
    let mx_1: Vec<Vec<f64>> = vec![
        vec![1.0, 3.0], 
        vec![2.0, 5.0]
    ];

    let mx1 = Matrix { main_matrix: mx_1 };

    let mx_2: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0], 
        vec![3.0, 4.0]
    ];

    let expected_result: Vec<Vec<f64>> = vec![
        vec![10.0, 14.0], 
        vec![17.0, 24.0]
    ];

    let result = mx1.mx_mult(mx_2);

    assert_eq!(result, Ok(expected_result.clone()), "Expected {:?}, but got {:?}", expected_result, result);
}



#[test]
fn scale_valid() {
    let mx_1: Vec<Vec<f64>> = vec![
        vec![1.0, 3.0], 
        vec![2.0, 5.0]
    ];

    let mx1 = Matrix { main_matrix: mx_1 };

    let scalar = 2.0;

    let expected_result: Vec<Vec<f64>> = vec![
        vec![2.0, 6.0], 
        vec![4.0, 10.0]
    ];

    let result = mx1.scale(scalar);

    assert_eq!(result, Ok(expected_result.clone()), "Expected {:?}, but got {:?}", expected_result, result);
}


#[test]
fn scale_invalid_mx() {
    let mx_1: Vec<Vec<f64>> = vec![];

    let mx1 = Matrix { main_matrix: mx_1 };

    let scalar = 2.0;

    let result = mx1.scale(scalar);

    assert_eq!(result, Err("Matrix cannot be empty".to_string()),
     "Expected 'Err(Matrix cannot be empty)', but got {:?}",result);
}

#[test]
fn scale_invalid_vectors() {
    let mx_1: Vec<Vec<f64>> = vec![vec![]];

    let mx1 = Matrix { main_matrix: mx_1 };

    let scalar = 2.0;

    let result = mx1.scale(scalar);

    assert_eq!(result, Err("Matrices cannot contain empty vectors".to_string()), 
    "Expected 'Err(Matrices cannot contain empty vectors)', but got {:?}", result);
}


// Matrix transpose tests FIX

fn vector_by_matrix() {
    let mx_1: Vec<Vec<f64>> = vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
        vec![5.0, 6.0]
    ];

    let mx1 = Matrix { main_matrix: mx_1 };

    let vector = vec![0.5, 0.5];
    let expected_result = vec![1.5, 3.5, 5.5];

    let result = mx1.vec_multiplication(vector).unwrap();

    assert_eq!(result, expected_result);
}