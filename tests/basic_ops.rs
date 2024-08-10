use machine_learning::algos::array_ops::*;



// Vector sum tests
#[test]
fn vector_sum1() {
    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let result = Matrix::vector_sum(vec1, vec2);
    assert_eq!(result, Some(vec![5.0, 7.0, 9.0]));
}

#[test]
fn vector_sum_uneven_len() {
    let vec1 = vec![1.0, 2.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let result = Matrix::vector_sum(vec1, vec2);
    assert_eq!(result, None);
}

// Matrix sum tests
#[test]
fn mx_sum() {
    let mx_1:Vec<Vec<f64>> = vec![
        vec![1.0,3.0], 
        vec![2.0,5.0]];
    
    let mx1 = Matrix{main_matrix:mx_1}; 

    let mx_2:Vec<Vec<f64>> = vec![
        vec![1.0,3.0], 
        vec![2.0,5.0]];

    
    assert_eq!(mx1.mx_sum(mx_2), 
        Some(vec![
            vec![2.0,6.0], 
            vec![4.0,10.0]]));
}