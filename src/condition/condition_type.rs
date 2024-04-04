use std::collections::HashMap;
use super::condition;

struct ConditionType {
    chain_type: String
} 

impl ConditionType {

    pub fn new(chain_type: String) -> Self {
        ConditionType { chain_type }
    }

    pub fn eveluate(&self, input: &HashMap<String, Box<dyn Any>>, condition: Vec<condition::Condition>) -> Result<bool, error> {
      match self.chain_type {
         "OR" => {
            Ok(true)
         }
      }
    }

    pub fn eveluateOR(&self,  input: HashMap<String, Box<dyn Any>>) -> Result<bool, error> {

    }
    
}



