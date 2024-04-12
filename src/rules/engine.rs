use std::collections::HashMap;
use crate::core::Rule;
use crate::condition::ConditionType;
use crate::function::Function;
use crate::function::aggregator::AggregateFunction;
use crate::function::list_function::ListFunctionConstraint;

struct Engine {
    rules: HashMap<String, Vec<Rule>>,
    condition_chains: HashMap<String, ConditionType>,
    input_functions: HashMap<String, Function>,
    aggregate_function: AggregateFunction,
    input_constraints: ListFunctionConstraint,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            
        }
    }
}