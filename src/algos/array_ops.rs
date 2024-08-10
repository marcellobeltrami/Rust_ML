pub struct Matrix {
    pub main_matrix: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn vector_sum(vec_1: Vec<f64>, vec_2: Vec<f64>) -> Option<Vec<f64>> {
        if vec_1.len() != vec_2.len() {
            return None; // Vectors must be the same length
        } else {
            let result = vec_1.iter().zip(vec_2.iter()).map(|(a, b)| a + b).collect();

            Some(result)
        }
    }

    // Matrix Sum
    pub fn mx_sum(&self, matrix_2: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
        if self.main_matrix.len() != matrix_2.len() {
            return None; // Vectors must be the same length
        } else {
            let mut result: Vec<Vec<f64>> = vec![];
            for (i, vector) in self.main_matrix.iter().enumerate() {
                if matrix_2.get(i).is_none() || vector.len() != matrix_2[i].len() {
                    return None; // Matrices must have the same row lengths
                }

                // Matches corresponding rows and carries out vector sum.
                match Self::vector_sum(vector.clone(), matrix_2[i].clone()) {
                    Some(sum_row) => result.push(sum_row),
                    None => return None, // If vector_sum fails, propagate None
                }
            }

            // returns summed matrix.
            Some(result)
        }
    }

    // Matrix Multiplication.
    pub fn mx_mult(&self, matrix_2: Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>>{
        todo!();
    }
}
