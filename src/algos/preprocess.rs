use csv::Reader;
use std::error::Error;
use std::collections::HashMap;
use std::vec;
use serde_json::Value;


pub struct Preprocess {}

impl  Preprocess{


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


    /// Read csv to JSON. 
    // Function to infer the data type
    fn infer_type(value: &str) -> Value {
        if let Ok(int_val) = value.parse::<i64>() {
            Value::Number(int_val.into())
        } else if let Ok(float_val) = value.parse::<f64>() {
            // Handle floating-point numbers
            Value::Number(serde_json::Number::from_f64(float_val).unwrap())
        } else if let Ok(bool_val) = value.parse::<bool>() {
            // Handle boolean values
            Value::Bool(bool_val)
        } else {
            // Fallback to string if no other types matched
            Value::String(value.to_string())
        }
    }

    fn csv_to_json(file_path: &str) -> Result<Value, Box<dyn Error>> {
        let mut rdr = Reader::from_path(file_path)?;
        
        // Get the headers
        let headers = rdr.headers()?.clone();
    
        // Create a HashMap to hold the data
        let mut data_map: HashMap<String, Vec<Value>> = HashMap::new();
    
        // Initialize the HashMap with empty vectors for each header
        for header in headers.iter() {
            data_map.insert(header.to_string(), Vec::new());
        }
    
        // Process each record
        for record in rdr.records() {
            let record = record?;
            for (header, value) in headers.iter().zip(record.iter()) {
                if let Some(vec) = data_map.get_mut(header) {
                    vec.push(Value::String(value.to_string()));
                }
            }
        }
    
        // Convert HashMap to serde_json::Value
        let json_value = Value::Object(data_map.into_iter()
            .map(|(k, v)| (k, Value::Array(v)))
            .collect());
    
        Ok(json_value)
    }
    
    // Function to convert serde_json::Value into a HashMap<String, Vec<Value>>
    fn json_to_hashmap(value: &Value) -> Result<HashMap<String, Vec<Value>>, Box<dyn Error>> {
        let mut hashmap: HashMap<String, Vec<Value>> = HashMap::new();
    
        if let Value::Object(map) = value {
            for (key, value) in map {
                if let Value::Array(vec) = value {
                    hashmap.insert(key.clone(), vec.to_vec());
                } else {
                    hashmap.insert(key.clone(), vec![value.clone()]);
                }
            }
        } else {
            return Err("Expected a JSON object".into());
        }
    
        Ok(hashmap)
    }
    


    // Function merging all preprocessed vector_numerics in one matrix. This should be called by the user. 
    pub fn import_csv(csv_file_path:&str) -> Result<Vec<Vec<f64>>, String> {
        
        // converts csv to json
        let json_data = Preprocess::csv_to_json(csv_file_path).unwrap();
        
        // converts json to a hashmap. 
        let hashmap: Result<HashMap<String, Vec<Value>>, Box<dyn Error>> = Preprocess::json_to_hashmap(&json_data);
        
        let mut _vector_numeric: Vec<f64> = vec![];

        
        match hashmap {
            Ok(hashmap) => {
                let mut vector_numeric: Vec<Vec<f64>> = vec![];
                for (_, vec) in hashmap {
                    let mut vec_f64: Vec<f64> = vec![];
                    for val in vec {
                        match val.as_f64() {
                            Some(f) => vec_f64.push(f),
                            None => return Err("Expected a JSON array of numbers".to_string()),
                        }
                    }
                    vector_numeric.push(vec_f64);
                }
                println!("Preprocessed {} labels", vector_numeric.len());
                Ok(vector_numeric)
            },
            Err(e) => Err(e.to_string()),
        }

    }



}




