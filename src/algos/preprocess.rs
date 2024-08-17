use csv::Reader;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::vec;

pub struct Preprocess {}

impl  Preprocess{


    // Load CSV file and return csv object. Refer to CSV documentation (https://docs.rs/csv/latest/csv/)
    fn load_csv(file_path: &str) -> Result<Reader<File>, Box<dyn Error>> {
        
        if !Path::new(file_path).exists() == false {
            return Err("File not found".to_string().into());
        }
        
        let rdr = ReaderBuilder::new().from_path(file_path)?;
        Ok(rdr)
    }

    fn vec_round(x: Vec<f64>, dec_points: f64) -> Vec<f64>{

        x.iter().map(|x| (x * dec_points).round() / dec_points).collect()

        
    }

    // converts intgers to floats
    fn int_to_float(x: Vec<i32>) -> Vec<f64>{
        x.iter().map(|x| *x as f64).collect()
    }

    // Scale vector_numerics of numerical values based on different methods 
    // Each function access the object, scales/encodes and returns a matrix 
    // with the numerical or categorical vector_numerics transformed.

    // All scaling results are rounded to 5 decimal places.
    pub fn log_scaling(vector_numeric: Vec<f64>) -> Result<Vec<f64>, String> {
        
        if vector_numeric.is_empty() == true {
            return Err("vector_numeric cannot be empty".to_string());
        }
        
        let result:Vec<f64> = vector_numeric.iter().map(|x| x.ln()).collect();
        
        let result_rounded = Preprocess::vec_round(result, 100000.0);
        
        Ok(result_rounded)
    }

    pub fn robust_scaling(vector_numeric: Vec<f64>) -> Result<Vec<f64>, String> {
        
        if vector_numeric.is_empty() == true { 
            return Err("vector_numeric cannot be empty".to_string());
        } 


        // Interpolation to obtain quartile values.        
        fn interpolate(data: &Vec<f64>, idx: f64) -> f64 {
            let lower_index = idx.floor() as usize; // rounds down
            let upper_index = idx.ceil() as usize; // rounds up

            if lower_index == upper_index {
                data[lower_index]
            } else {
                let lower_value = data[lower_index];
                let upper_value = data[upper_index];
                lower_value + (upper_value - lower_value) * (idx - lower_index as f64)
            }
        }


        let mut sorted_vector = vector_numeric.clone();
        sorted_vector.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
 
        let  median: f64;
        let  upper_quartile: f64;
        let  low_quartile: f64;

        if (vector_numeric.len())%2 == 0 { //even vector 

            median = (sorted_vector[vector_numeric.len()/2] + sorted_vector[vector_numeric.len()/2 - 1]) / 2.0;
            
            let low_idx = 0.25 * ((vector_numeric.len() -1) as f64);
            let up_idx = 0.75 * ((vector_numeric.len() -1) as f64);

            upper_quartile = interpolate(&sorted_vector, up_idx);
            low_quartile = interpolate(&sorted_vector, low_idx);
            
        } else {
            median = sorted_vector[vector_numeric.len()/2]; // uneven vector
            

            let low_idx = 0.25 * ((vector_numeric.len() -1) as f64);
            let up_idx = 0.75 * ((vector_numeric.len() -1) as f64);

            upper_quartile = interpolate(&sorted_vector, up_idx);
            low_quartile = interpolate(&sorted_vector, low_idx);
        }

        // calculate robust scaling values returninh them as a vector in their original position.
        let result:Vec<f64> = vector_numeric.iter().map(|x| (x - median) / (upper_quartile - low_quartile)).collect();
        
        let res_rounded = Preprocess::vec_round(result, 100000.0); // rounds vecor to 5 decimal places

        Ok(res_rounded)

    }

    pub fn standard_scaling(vector_numeric: Vec<f64>) -> Result<Vec<f64>, String> {
        
        if vector_numeric.is_empty() == true {
            return Err("vector_numeric cannot be empty".to_string());
        }
        
        let mean = vector_numeric.iter().sum::<f64>() / vector_numeric.len() as f64;
        let std_dev = (vector_numeric.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / vector_numeric.len() as f64).sqrt();
        let result = vector_numeric.iter().map(|x| (x - mean) / std_dev).collect();

        let res_rounded = Preprocess::vec_round(result, 10000.0);
        Ok(res_rounded)
    }
    
    
    // Encoding methods for categorical data.
    pub fn label_encoding(vector_categorical: Vec<String>) -> Result<Vec<f64>, String> {
        
        if vector_categorical.is_empty() == true {
            return Err("vector_categorical cannot be empty".to_string());
        }
        
        
        let mut hash_map: HashMap<String, usize> = HashMap::new();
        for (i, val) in vector_categorical.iter().enumerate() {
            hash_map.entry(val.to_string()).or_insert(i);
        }

        let mut  encoded_vector:Vec<f64> = vec![];
        for i in vector_categorical.iter() {
            if !hash_map.contains_key(i) {
                return Err("Invalid label".to_string());
            } else {
                encoded_vector.push(hash_map[i] as f64);
            }

        }
       
        if encoded_vector.len() != vector_categorical.len() {
            return Err("Something went wrong, encoded vector is not the same length as the original".to_string());
        }

        Ok(encoded_vector)
    }


    // Function merging all preprocessed vector_numerics in one matrix. This should be called by the user. 
    pub fn auto_preprocess(csv_file_path:&str, encoding_method: &str, scaling_method: &str) -> Result<Vec<Vec<f64>>, String> {
        let mut  csv_obj = Preprocess::load_csv(csv_file_path).unwrap();
        
        for result in csv_obj.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            let record = result;
            println!("{:?}", record);
        }


        todo!()
    }



}




