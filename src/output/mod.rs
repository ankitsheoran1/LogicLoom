use serde_json::{Map, Value};
use std::collections::HashMap;



fn build_output(output_results: Vec<HashMap<String, Value>>) -> Result<Map<String, Value>, Box<dyn std::error::Error>> {
    let mut output = Map::new();

    for result in output_results {
        for (k, v) in result {
            set_output_value(&mut output, &k, v);
        }
    }

    Ok(output)
}


fn set_output_value(m: &mut Map<String, Value>, path: &str, val: Value) {
    if let Some(dot_index) = path.find('.') {
        // If the path contains a dot, split it into the current key and the remaining path.
        let (current_key, remaining_path) = path.split_at(dot_index);
        let current_key = current_key.to_owned();
        let remaining_path = &remaining_path[1..]; // Skip the dot.

        // Ensure there is a sub-map to recurse into.
        if !m.contains_key(&current_key) {
            m.insert(current_key.clone(), Value::Object(Map::new()));
        }

        // Recurse into the sub-map.
        if let Some(Value::Object(sub_map)) = m.get_mut(&current_key) {
            set_output_value(sub_map, remaining_path, val);
        }
    } else {
        // If the path does not contain a dot, set the value directly.
        match m.get_mut(path) {
            Some(existing_val) if existing_val.is_array() => {
                // If the existing value is an array, push the new value into it.
                //existing_val.as_array_mut().unwrap().push(val);
                if let Value::Array(new_arr) = val {
                    existing_val.as_array_mut().unwrap().extend(new_arr);
                } else {
                    // If the new value is not an array, just push it into the existing array.
                    existing_val.as_array_mut().unwrap().push(val);
                }
            }
            _ => {
                // Otherwise, insert or overwrite the value at the path.
                m.insert(path.to_owned(), val);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;
    use serde_yaml;
    #[test]
fn test_build_output() {
    // Setup the input data similar to the Go test case.
    let mut map1 = HashMap::new();
    map1.insert("file.priority".to_string(), Value::String("low".to_string()));

    let mut map2 = HashMap::new();
    map2.insert("file.priority".to_string(), Value::String("high".to_string()));

    let mut map3 = HashMap::new();
    map3.insert("nested.value.here".to_string(), Value::String("A".to_string()));

    let mut map4 = HashMap::new();
    map4.insert("nested.slice.here".to_string(), Value::Array(vec![
        Value::String("A".to_string()),
        Value::String("B".to_string()),
        Value::String("C".to_string()),
    ]));

    let mut map5 = HashMap::new();
    map5.insert("nested.slice.here".to_string(), Value::Array(vec![
        Value::String("D".to_string()),
        Value::String("E".to_string()),
    ]));

    let mut map6 = HashMap::new();
    map6.insert("nested.slice.here".to_string(), Value::Array(vec![
        Value::String("F".to_string()),
        Value::String("G".to_string()),
    ]));

    let mut map7 = HashMap::new();
    map7.insert("numbers.here".to_string(), Value::Array(vec![
        Value::Number(serde_json::Number::from(11)),
        Value::Number(serde_json::Number::from(22)),
    ]));

    let mut map8 = HashMap::new();
    map8.insert("numbers.here".to_string(), Value::Array(vec![
        Value::Number(serde_json::Number::from(33)),
        Value::Number(serde_json::Number::from(44)),
    ]));

    let results = vec![map1, map2, map3, map4, map5, map6, map7, map8];
    

    // Define the expected YAML output as a string.
    let expected_yaml_output = r#"---
file:
  priority: high
nested:
  slice:
    here:
      - A
      - B
      - C
      - D
      - E
      - F
      - G
  value:
    here: A
numbers:
  here:
    - 11
    - 22
    - 33
    - 44
"#;

    // Call the build_output function to process the results.
    let output = build_output(results).expect("expected no error when building output");

    println!("{:?}", output);

    // Serialize the output to YAML.
    let mut yaml_output = serde_yaml::to_string(&output).expect("expected no error when marshaling output into YAML");
    if !yaml_output.starts_with("---") {
        yaml_output.insert_str(0, "---\n");
    }

    println!("{:?}", yaml_output);

    // Compare the actual output with the expected output.
    assert_eq!(expected_yaml_output, yaml_output, "expected yaml output to be as the defined one");
}
}