use std::collections::HashMap;
use std::any::Any;
use std::error::Error;

struct Condition {
    input_path: String,
    functions: Vec<fn(&HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>>>,
}

impl Condition {
    fn new(input_path: String, functions: Vec<fn(&HashMap<String, Box<dyn Any>>)-> Result<bool, Box<dyn Error>>>) -> Self {
        Condition {
            input_path,
            functions
        }

    }

    fn eval(&self, input: &HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>> {
        for function in &self.functions {
            let result = function(input)?;
            if !result {
                return Ok(false);
            }
        }
        Ok(true)
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::any::Any;

    // A helper function that always returns Ok(true)
    fn always_true(_: &HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>> {
        Ok(true)
    }

    // A helper function that always returns Ok(false)
    fn always_false(_: &HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>> {
        Ok(false)
    }

    fn sum_values(input: &HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>> {
        let sum = input
            .values()
            .try_fold(0, |acc, value| {
                if let Some(number) = value.downcast_ref::<i32>() {
                    Ok(acc + number)
                } else {
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "value is not an i32")))
                }
            })?;
    
        Ok(sum == 4) // We expect the sum to be 4 for the test case
    }

    // A test case for when all functions return true
    #[test]
    fn evaluate_all_true() {
        let condition = Condition::new(
            "input_path".to_string(),
            vec![always_true as fn(&HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>>],
        );
        let input = HashMap::new();
        assert_eq!(condition.eval(&input).unwrap(), true);
    }

    // A test case for when one function returns false
    #[test]
    fn evaluate_one_false() {
        let condition = Condition::new(
            "input_path".to_string(),
            vec![always_true, always_false],
        );
        let input = HashMap::new();
        assert_eq!(condition.eval(&input).unwrap(), false);
    }

    // A test case for when a function returns an error
    #[test]
    fn evaluate_function_error() {
        fn error_function(_: &HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>> {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "error")))
        }

        let condition = Condition::new(
            "input_path".to_string(),
            vec![always_true, error_function],
        );
        let input = HashMap::new();
        assert!(condition.eval(&input).is_err());
    }

    #[test]
    fn evaluate_sum_of_values() {
        let mut input = HashMap::new();
        input.insert("key1".to_string(), Box::new(2) as Box<dyn Any> );
        input.insert("key2".to_string(), Box::new(2) as Box<dyn Any> );

        let condition = Condition::new(
            "input_path".to_string(),
            vec![sum_values],
        );

        assert_eq!(condition.eval(&input).unwrap(), true);
    }
}






