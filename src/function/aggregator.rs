use std::collections::HashMap;
use std::error::Error;
use std::any::Any;

#[derive(Eq, PartialEq, Hash)]
pub enum Aggregator {
    SUM,
    MIN,
    MAX,
    AVG,
    COUNT,
}

fn parse_inputs_to_numbers(inputs: &[Box<dyn Any>]) -> Result<Vec<f64>, Box<dyn Error>> {
    if inputs.is_empty() {
        return Err("no inputs provided".into());
    }

    let mut inputs_nr = Vec::new();
    for input in inputs {
        let input_str = input.downcast_ref::<String>()
            .ok_or_else(|| "input is not a string")?;
        let input_nr = input_str.parse::<f64>()
            .map_err(|err| format!("could not convert input [{}] to number: {}", input_str, err))?;
        inputs_nr.push(input_nr);
    }

    Ok(inputs_nr)
}

type AggregateFunction = Box<dyn Fn(Vec<Box<dyn Any>>) -> Result<f64, Box<dyn Error>>>;

fn default_aggregate_function() -> HashMap<Aggregator, AggregateFunction> {
    let mut m: HashMap< Aggregator, AggregateFunction>  = HashMap::new();

    m.insert(Aggregator::SUM, Box::new(|input: Vec<Box<dyn Any>>| {
        let inputs_as_numbers = parse_inputs_to_numbers(&input)?;
        let sum: f64 = inputs_as_numbers.iter().sum();
        Ok(sum)

    }));

    m.insert(Aggregator::AVG, Box::new(|input: Vec<Box<dyn Any>>| {
        let inputs_as_numbers = parse_inputs_to_numbers(&input)?;
        let sum: f64 = inputs_as_numbers.iter().sum();
        Ok( sum / input.len() as f64)
    }));

    m.insert(Aggregator::MAX, Box::new(|input: Vec<Box<dyn Any>>| {
        let inputs_as_numbers = parse_inputs_to_numbers(&input)?;
        let mx = inputs_as_numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        Ok(mx)
    }));

    m.insert(Aggregator::MIN, Box::new(|input: Vec<Box<dyn Any>>| {
        let inputs_as_numbers = parse_inputs_to_numbers(&input)?;
        let mn = inputs_as_numbers.iter().fold(f64::NEG_INFINITY, |a, &b| a.min(b));
        Ok(mn)
    }));

    m.insert(Aggregator::COUNT, Box::new(|input: Vec<Box<dyn Any>>| {
      Ok(input.len() as f64)
    }));
     m
}