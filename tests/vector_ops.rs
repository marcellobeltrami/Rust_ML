use machine_learning::algos::array_ops::*;

// Vector operations tests
#[test]
fn vector_sum1() {
    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let result = Vector::vector_sum(vec1, vec2);
    assert_eq!(result, Some(vec![5.0, 7.0, 9.0]));
}

#[test]
fn vector_sum_large_numbers() {
    let vec1 = vec![1_000_000_000.0, 2_000_000_000.0, 3_000_000_000.0];
    let vec2 = vec![4_000_000_000.0, 5_000_000_000.0, 6_000_000_000.0];
    let result = Vector::vector_sum(vec1, vec2);
    assert_eq!(
        result,
        Some(vec![5_000_000_000.0, 7_000_000_000.0, 9_000_000_000.0])
    );
}

#[test]
fn vector_sum_uneven_len() {
    let vec1 = vec![1.0, 2.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let result = Vector::vector_sum(vec1, vec2);
    assert_eq!(result, None);
}

#[test]
fn vector_dot1() {
    let vec1 = vec![1.0, 2.0, 7.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let result = Vector::vector_dot(vec1, vec2);
    assert_eq!(result, Some(56.0));
}

#[test]
fn vector_dot_uneven_len() {
    let vec1 = vec![1.0, 2.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let result = Vector::vector_dot(vec1, vec2);
    assert_eq!(result, None);
}

#[test]
fn vector_dot_large_numbers() {
    let vec1 = vec![1_000_000_000.0, 2_000_000_000.0, 3_000_000_000.0];
    let vec2 = vec![4_000_000_000.0, 5_000_000_000.0, 6_000_000_000.0];
    let result = Vector::vector_dot(vec1, vec2);
    assert_eq!(result, Some(32_000_000_000_000_000_000.0));
}

#[test]
fn vector_mult_diff_len() {
    let vec1 = vec![1.0, 2.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let result = Vector::vector_mult(vec1, vec2);
    assert_eq!(
        result,
        Some(vec![vec![4.0, 5.0, 6.0], vec![8.0, 10.0, 12.0]])
    );
}

#[test]
fn vector_mult_same_len() {
    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let result = Vector::vector_mult(vec1, vec2);
    assert_eq!(
        result,
        Some(vec![
            vec![4.0, 5.0, 6.0],
            vec![8.0, 10.0, 12.0],
            vec![12.0, 15.0, 18.0]
        ])
    );
}
#[test]
fn test_norm_euclidean_positive() {
    let vec = vec![3.0, 4.0];
    let result = Vector::norm_euclidean(vec);
    assert_eq!(result, Some(5.0), "Expected Some(5.0) but got {:?}", result);
}

#[test]
fn test_norm_euclidean_negative() {
    let vec = vec![-1.0, -2.0, -2.0];
    let result = Vector::norm_euclidean(vec);
    assert!((result.unwrap() - 3.0).abs() < 1e-10, "Expected Some(3.0) but got {:?}", result);
}

#[test]
fn test_norm_euclidean_mixed() {
    let vec = vec![1.0, -2.0, -4.0, 5.0, 6.0];
    let result = Vector::norm_euclidean(vec);
    assert!((result.unwrap() - 9.055385138137417).abs() < 1e-10, "Expected Some(9.055) but got {:?}", result);
}

#[test]
fn test_norm_euclidean_empty() {
    let vec: Vec<f64> = vec![];
    let result = Vector::norm_euclidean(vec);
    assert_eq!(result, None, "Expected None but got {:?}", result);
}

#[test]
fn test_norm_manhattan_positive() {
    let vec = vec![3.0, 4.0];
    let result = Vector::norm_manhattan(vec);
    assert_eq!(result, Some(7.0), "Expected Some(7.0) but got {:?}", result);
}

#[test]
fn test_norm_manhattan_negative() {
    let vec = vec![-1.0, -2.0, -3.0];
    let result = Vector::norm_manhattan(vec);
    assert!((result.unwrap() - 6.0).abs() < 1e-10, "Expected Some(6.0) but got {:?}", result);
}

#[test]
fn test_norm_manhattan_mixed() {
    let vec = vec![1.0, -2.0, -4.0, 5.0, 6.0];
    let result = Vector::norm_manhattan(vec);
    assert!((result.unwrap() - 18.0).abs() < 1e-10, "Expected Some(18.0) but got {:?}", result);
}

#[test]
fn test_norm_manhattan_empty() {
    let vec: Vec<f64> = vec![];
    let result = Vector::norm_manhattan(vec);
    assert_eq!(result, None, "Expected None but got {:?}", result);
}
#[test]
fn test_norm_infinity_positive() {
    let vec = vec![3.0, 4.0, 7.0];
    let result = Vector::norm_infinity(vec);
    assert_eq!(result, Some(7.0), "Expected Some(7.0) but got {:?}", result);
}

#[test]
fn test_norm_infinity_negative() {
    let vec = vec![-1.0, -2.0, -3.0];
    let result = Vector::norm_infinity(vec);
    assert_eq!(result, Some(3.0), "Expected Some(3.0) but got {:?}", result);
}

#[test]
fn test_norm_infinity_mixed() {
    let vec = vec![1.0, -2.0, -4.0, 5.0, 6.0];
    let result = Vector::norm_infinity(vec);
    assert_eq!(result, Some(6.0), "Expected Some(6.0) but got {:?}", result);
}

#[test]
fn test_norm_infinity_empty() {
    let vec: Vec<f64> = vec![];
    let result = Vector::norm_infinity(vec);
    assert_eq!(result, None, "Expected None but got {:?}", result);
}

#[test]
fn test_norm_infinity_single_element() {
    let vec = vec![42.0];
    let result = Vector::norm_infinity(vec);
    assert_eq!(result, Some(42.0), "Expected Some(42.0) but got {:?}", result);
}