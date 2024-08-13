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
