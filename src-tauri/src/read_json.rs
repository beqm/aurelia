use serde_json::Value;
use std::fs::File;
use std::io::{Error, Read};

fn extract_strings(json_value: &Value, string_vec: &mut Vec<String>) {
    match json_value {
        Value::String(s) => {
            // If the value is a string, add its lowercase version to the vector
            string_vec.push(s.to_lowercase());
        }
        Value::Array(arr) => {
            // If the value is an array, recursively call the function for each element
            for item in arr {
                extract_strings(item, string_vec);
            }
        }
        Value::Object(map) => {
            // If the value is an object, recursively call the function for each value
            for value in map.values() {
                extract_strings(value, string_vec);
            }
        }
        _ => {
            // Ignore other types
        }
    }
}

pub fn read_json_file(file_path: &str) -> Result<Vec<String>, Error> {
    // Open the file in read-only mode
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a serde_json::Value
    let json_value: Value = serde_json::from_str(&contents)?;

    // Initialize the vector to store strings
    let mut string_vec = Vec::new();

    // Extract strings from the JSON data
    extract_strings(&json_value, &mut string_vec);

    Ok(string_vec)
}
