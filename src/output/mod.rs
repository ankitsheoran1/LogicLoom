use serde_json::{Map, Value};

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
                existing_val.as_array_mut().unwrap().push(val);
            }
            _ => {
                // Otherwise, insert or overwrite the value at the path.
                m.insert(path.to_owned(), val);
            }
        }
    }
}