use machine_learning::algos::array_ops::*;

// Matrix sum tests
#[test]
fn mx_sum() {
    let mx_1: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![2.0, 5.0]];

    let mx1 = Matrix { main_matrix: mx_1 };
    let mx_2: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![2.0, 5.0]];

    assert_eq!(
        mx1.mx_sum(mx_2),
        Some(vec![vec![2.0, 6.0], vec![4.0, 10.0]])
    );
}

#[test]
fn mx_sum_uneven() {
    let mx_1: Vec<Vec<f64>> = vec![vec![1.0, 3.0], vec![2.0, 5.0]];

    let mx1 = Matrix { main_matrix: mx_1 };

    let mx_2: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0, 5.0]];

    assert_eq!(mx1.mx_sum(mx_2), None);
}
