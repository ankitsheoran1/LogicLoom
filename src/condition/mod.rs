use std::collections::HashMap;
use std::any::Any;
use std::error::Error;

pub struct Condition {
    input_path: String,
    functions: Vec<fn(&HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>>>,
}

impl Condition {
    pub fn new(input_path: String, functions: Vec<fn(&HashMap<String, Box<dyn Any>>)-> Result<bool, Box<dyn Error>>>) -> Self {
        Condition {
            input_path,
            functions
        }

    }

    pub fn eval(&self, input: &HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>> {
        for function in &self.functions {
            let result = function(input)?;
            if !result {
                return Ok(false);
            }
        }
        Ok(true)
    }


}

enum LogicalOperator {
    OR,
    AND,
}

struct ConditionType {
    chain_type: LogicalOperator
} 

impl ConditionType {

    pub fn new(chain_type: LogicalOperator) -> Self {
        ConditionType { chain_type }
    }

    pub fn evaluate(&self, input: &HashMap<String, Box<dyn Any>>, condition: Vec<Condition>) -> Result<bool, Box<dyn Error>> {
      match self.chain_type {
         LogicalOperator::OR => {
            self.evaluate_or(input, condition)
         }
         LogicalOperator::AND => {
           self.evaluate_and(input, condition)
         }
         

      }
    }

    pub fn evaluate_or(&self,  input: &HashMap<String, Box<dyn Any>>, conditions: Vec<Condition>) -> Result<bool, Box<dyn Error>> {
        for condition in conditions {
            println!("Evaluating condition with input_path: {}", condition.input_path);
            if condition.eval(input)? {
                return Ok(true);
            }
        }
        Ok(false)
       
    }

    pub fn evaluate_and(&self,  input: &HashMap<String, Box<dyn Any>>, conditions: Vec<Condition>) -> Result<bool, Box<dyn Error>> {
        let mut count = conditions.len();
        for condition in conditions {
            if condition.eval(input)? {
                 count -= 1;
            }
        }

        Ok(count == 0)
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

    fn age_less_than_30(input: &HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>> {
        if let Some(age) = input.get("age").and_then(|v| v.downcast_ref::<i32>()) {
            Ok(*age < 30)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "age is not an i32")))
        }
    }

    fn name_length_greater_than_10(input: &HashMap<String, Box<dyn Any>>) -> Result<bool, Box<dyn Error>> {
        if let Some(name) = input.get("name").and_then(|v| v.downcast_ref::<String>()) {
            Ok(name.len() > 10)
        } else {
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "name is not a String")))
        }
    }

    #[test]
    fn evaluate_or_condition() {
        let condition_age = Condition::new("age".to_string(), vec![age_less_than_30]);
        let condition_name = Condition::new("name".to_string(), vec![name_length_greater_than_10]);

        let mut input = HashMap::new();
        input.insert("name".to_string(), Box::new("Ankit".to_string()) as Box<dyn Any>);
        input.insert("age".to_string(), Box::new(28) as Box<dyn Any>);

        let condition_type = ConditionType::new(LogicalOperator::OR);
        let result = condition_type.evaluate(&input, vec![condition_age, condition_name]);

        assert_eq!(result.unwrap(), true);
    }
}






