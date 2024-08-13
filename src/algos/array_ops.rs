use std::vec;

// Linear Algebra Vector operations
pub struct Vector {}

impl Vector {
    // Vector sum
    pub fn vector_sum(vec_1: Vec<f64>, vec_2: Vec<f64>) -> Result<Vec<f64>, String> {
        if vec_1.len() != vec_2.len() {
            return Err("Vectors must have equal lengths".to_string()); // Vectors must be the same length
        } else {
            let mut result = vec![];

            for (i, num1) in vec_1.iter().enumerate() {
                let sum = num1 + vec_2[i];

                result.push(sum);
            }

            return Ok(result);
        }
    }

    // Scalar multiplication
    pub fn scalar_mult(vec1: Vec<f64>, scalar_val: f64) -> Result<Vec<f64>, String> {
        let mut res_vec: Vec<f64> = vec![];
        if vec1.is_empty() == true {
            return Err("Vector cannot be empty".to_string()); // Vector Empty
        } else {
            for i in vec1 {
                res_vec.push(i * scalar_val);
            }

            return Ok(res_vec);
        }
    }

    // Vector Dot Product
    pub fn vector_dot(vec_1: Vec<f64>, vec_2: Vec<f64>) -> Result<f64, String> {
        if vec_1.len() != vec_2.len() {
            return Err("Vectors must have equal length".to_string()); // Vectors must be the same length
        } else if vec_1.is_empty() || vec_2.is_empty(){
            return Err("Vector cannot be empty".to_string())
        } else {
            let mut dot_pr = 0.0;
            for (i, num1) in vec_1.iter().enumerate() {
                let mult = num1 * vec_2[i];

                dot_pr += mult
            }
            return Ok(dot_pr);
        }
    }

    // Euclidean Norm
    pub fn norm_euclidean(vec1: Vec<f64>) -> Result<f64, String> {
        if vec1.is_empty() == true {
            return Err("Vector cannot be empty".to_string());
        } else {
            let powered_vec: f64 = vec1.iter().map(|&a| a * a).sum();
            return Ok(powered_vec.sqrt());
        }
    }

    // Manhattan Norm
    pub fn norm_manhattan(vec1: Vec<f64>) -> Result<f64, String> {
        if vec1.is_empty() == true {
            return Err("Vector cannot be empty".to_string());
        } else {
            let mut run_sum = 0.0;
            for i in vec1 {
                run_sum += i.abs();
            }
            return Ok(run_sum);
        }
    }

    // Infinity Norm
    pub fn norm_infinity(vec1: Vec<f64>) -> Result<f64, String> {
        if vec1.is_empty() == true {
            Err("Vector cannot be empty".to_string())
        } else {
            // Magic spell to find the max value of a vector.
            fn max_of_vector(v: &[f64]) -> Option<f64> {
                v.iter().fold(None, |max, &x| match max {
                    Some(m) if x > m => Some(x),
                    _ => max.or(Some(x)),
                })
            }

            // Actual norm infinity implementation
            let mut abs_vec = vec![];
            for i in vec1 {
                abs_vec.push(i.abs())
            }

            match max_of_vector(&abs_vec) {
                Some(max) => return Ok(max),
                None => return Err("Unknown error has occurred".to_string()),
            }
        }
    }

    // Vector multiplication. Return a matrix with multiplied vectors.
    pub fn vector_mult(vec1: Vec<f64>, vec2: Vec<f64>) -> Result<Vec<Vec<f64>>, String> {
        
        if vec1.is_empty() || vec2.is_empty() {
            return Err("Vectors cannot be empty".to_string())
        }
        
        let mut mult_vc: Vec<Vec<f64>> = vec![];

        for num in vec1.iter() {
            let vec_tmp = vec2.iter().map(|&a| a * num).collect();

            mult_vc.push(vec_tmp);
        }

        return Ok(mult_vc);
    }
}

// Linear Algebra Matrix operations
pub struct Matrix {
    pub main_matrix: Vec<Vec<f64>>,
}

impl Matrix {
    // Matrix Subtract
    pub fn mx_subtract(&self, matrix_2: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
        todo!();
    }

    // Matrix Sum
    pub fn mx_sum(&self, matrix_2: Vec<Vec<f64>>) -> Result<Vec<Vec<f64>>, String> {
        if self.main_matrix.len() != matrix_2.len() {
            return Err("Matrices must have same shape".to_string()); // Vectors must be the same length
        } else {
            let mut result: Vec<Vec<f64>> = vec![];
            for (i, vector) in self.main_matrix.iter().enumerate() {
                if matrix_2.get(i).is_none() || vector.len() != matrix_2[i].len() {
                    return Err("Matrices must have the same row lengths".to_string()); // Matrices must have the same row lengths
                }

                // Matches corresponding rows and carries out vector sum.
                match Vector::vector_sum(vector.clone(), matrix_2[i].clone()) {
                    Ok(sum_row) => result.push(sum_row),
                    Err(e) => return Err(e), // Pass along the error from vector_sum
                }
            }

            // returns summed matrix.
            Ok(result)
        }
    }

    pub fn mx_scalar(&self, scalar_val: f64) -> Option<Vec<Vec<f64>>> {
        todo!();
    }

    // Matrix Multiplication.
    pub fn mx_mult(&self, matrix_2: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
        todo!();
    }
}
