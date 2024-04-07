use std::collections::HashMap;
use std::error::Error;
use std::any::Any;
use std::io;

pub  mod list_function;
pub mod aggregator;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum SingleInputFunction {
   Empty, 
   NonEmpty,
   Equal,
   Greater, 
   GreaterEq, 
   Lower,
   LowerEq,
   Between,
   BetweenEq,
   NotBetween,
   NotBetweenEq,
   EqualIgnoreCase,
   EqualAny,
   EqualAnyIgnoreCase,
   NotEqualAny,
   StartsWith,
   StartsWithIgnoreCase,
   EndsWith,
   EndsWithIgnoreCase,
   Contains,
   ContainsIgnoreCase
}

type Function = Box<dyn Fn(Box<dyn Any>, Vec<Box<dyn Any>>) -> Result<bool, Box<dyn Error>> + 'static>;

fn default() -> HashMap<SingleInputFunction, Function> {
    let mut map: HashMap<SingleInputFunction, Function>  = HashMap::new();
    default_general_functions(&mut map);
    default_numeric_function(&mut map);
    default_string_function(&mut map);
    map
}

fn default_general_functions(m : &mut HashMap<SingleInputFunction, Function>) -> &mut HashMap<SingleInputFunction, Function> {
   // let mut m = HashMap::new();
    
    m.insert(SingleInputFunction::Empty, Box::new(|input: Box<dyn Any>, _args: Vec<Box<dyn Any>>| {
        match input.downcast_ref::<String>() {
            Some(string) => Ok(string.is_empty()),
            None => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Input is not a string")) as Box<dyn Error>),
        }
    })as Function);

    m.insert(SingleInputFunction::NonEmpty, Box::new(|input: Box<dyn Any>, _args: Vec<Box<dyn Any>>| {
        match input.downcast_ref::<String>() {
            Some(string) => Ok(string.is_empty() == false),
            None => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Input is not a string")) as Box<dyn Error>),
            
        }
    }));

    m.insert(SingleInputFunction::Equal, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        if args.len() != 1 {
            return Err(Box::new(io::Error::new(io::ErrorKind::Other, "Equal: needs one argument")));
        }
        let input_str = input.downcast_ref::<String>().ok_or_else(|| {
            Box::new(io::Error::new(io::ErrorKind::Other, "Input is not a string"))
        })?;
        let arg_str = args[0].downcast_ref::<String>().ok_or_else(|| {
            Box::new(io::Error::new(io::ErrorKind::Other, "Argument is not a string"))
        })?;
        Ok(input_str == arg_str)
    }));
    
    m
}

fn default_numeric_function(m: &mut HashMap<SingleInputFunction, Function>) -> &mut HashMap<SingleInputFunction, Function>{

    m.insert(SingleInputFunction::Greater, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(SingleInputFunction::Greater, &*input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num > args_no[0]),
            Err(e) => Err(e)
        }
    }));

    m.insert(SingleInputFunction::GreaterEq, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(SingleInputFunction::GreaterEq, &*input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num >= args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(SingleInputFunction::Lower, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(SingleInputFunction::Lower, &*input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num < args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(SingleInputFunction::LowerEq, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(SingleInputFunction::LowerEq, &*input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num <= args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(SingleInputFunction::Between, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(SingleInputFunction::Between, &*input, &args_refs, 2) {
            Ok((num, args_no)) =>  Ok(num > args_no[0] && num < args_no[1]),
            Err(e) => Err(e)
            
        }
    }));
    
    m.insert(SingleInputFunction::BetweenEq, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(SingleInputFunction::BetweenEq, &*input, &args_refs, 2) {
            Ok((num, args_no)) =>  Ok(num >= args_no[0] && num <= args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(SingleInputFunction::NotBetween, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(SingleInputFunction::Between, &*input, &args_refs, 2) {
            Ok((num, args_no)) =>  Ok(num <= args_no[0] || num >= args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(SingleInputFunction::NotBetweenEq, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(SingleInputFunction::NotBetweenEq, &*input, &args_refs, 2) {
            Ok((num, args_no)) =>  Ok(num > args_no[0] && num < args_no[0]),
            Err(e) => Err(e)
            
        }
    }));


    m

}

fn parse_numeric( function_name: SingleInputFunction ,
    input: &dyn Any,
    args: &[&dyn Any],
    required_args_count: usize) -> Result<(f64, Vec<f64>), Box<dyn Error>> {
        let input_ref = input.downcast_ref::<String>().ok_or_else(|| {
            format!(
                "[{:?}]: could not convert input [{:?}] to number: input is not a string",
                function_name, input
            )
        })?;
        let mut input_no = input_ref.parse::<f64>().map_err(|e| {
            format!("Failed to parse string as f64: {}", e)

        })?;
        match input_ref.parse::<f64>() {
            Ok(number) =>  { input_no = number; },
            Err(e) => println!("Failed to parse string as f64: {}", e)

        }

        if args.len() != required_args_count {
            return Err(format!(
                "[{:?}]: not enough arguments provided, expected {} but got {}",
                function_name,
                required_args_count,
                args.len()
            )
            .into());
        }
        let mut args_nr = Vec::with_capacity(required_args_count);
        for (i, &arg) in args.iter().enumerate() {
            let arg_ref = arg.downcast_ref::<String>().ok_or_else(|| {
                format!(
                    "[{:?}]: could not convert argument [{}] [{:?}] to number: argument is not a string",
                    function_name, i, arg
                )
            })?;
             match arg_ref.parse::<f64>() {
                 Ok(num) =>  {  args_nr.push(num); }
                 Err(err) =>   { return Err(format!(
                    "[{:?}]: could not convert argument [{}] [{:?}] to number: {}",
                    function_name, i, arg_ref, err
                ).into()); }

            };
        }
        Ok((input_no, args_nr))
}



fn default_string_function(m: &mut HashMap<SingleInputFunction, Function>)  -> &mut HashMap<SingleInputFunction, Function> {
    m.insert(SingleInputFunction::EqualIgnoreCase, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EqualIgnoreCase: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.eq_ignore_ascii_case(arg_str))
    }));

    m.insert(SingleInputFunction::EqualAnyIgnoreCase, Box::new(|input, args| {
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        for arg in args {
            if let Some(arg_str) = arg.downcast_ref::<String>() {
                if input_str.eq_ignore_ascii_case(arg_str) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }));

    m.insert(SingleInputFunction::EqualAny, Box::new(|input, args| {
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        for arg in args {
            if let Some(arg_str) = arg.downcast_ref::<String>() {
                if input_str == arg_str {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }));

    m.insert(SingleInputFunction::NotEqualAny, Box::new(|input, args| {
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        for arg in args {
            if let Some(arg_str) = arg.downcast_ref::<String>() {
                if input_str == arg_str {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }));

    m.insert(SingleInputFunction::StartsWith, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("StartsWith: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.starts_with(arg_str))
    }));

    m.insert(SingleInputFunction::StartsWithIgnoreCase, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("StartsWith: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.to_lowercase().starts_with(&arg_str.to_lowercase()))
    }));

    m.insert(SingleInputFunction::EndsWith, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EndsWith: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.ends_with(arg_str))
    }));

    m.insert(SingleInputFunction::EndsWithIgnoreCase, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EndsWithIgnoreCase: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.to_lowercase().ends_with(&arg_str.to_lowercase()))
    }));

    m.insert(SingleInputFunction::Contains, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EndsWithIgnoreCase: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.contains(arg_str))
    }));
    m.insert(SingleInputFunction::ContainsIgnoreCase, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EndsWithIgnoreCase: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.to_lowercase().contains(&arg_str.to_lowercase()))
    }));
    
    m

}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_default_single_input_functions() {
        let default_functions = default();

        let result = default_functions[&SingleInputFunction::Empty](Box::new(String::from("")), vec![]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::NonEmpty](Box::new(String::from("ankit")), vec![]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::Equal](Box::new(String::from("ankit")), vec![Box::new(String::from("ankit"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::Equal](Box::new(String::from("ankit")), vec![Box::new(String::from("anki"))]);
        assert!(result.is_ok_and(|x| x == false));
    
        let result = default_functions[&SingleInputFunction::Greater](Box::new(String::from("1")), vec![Box::new(String::from("0"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::GreaterEq](Box::new(String::from("1")), vec![Box::new(String::from("1"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::Lower](Box::new(String::from("0")), vec![Box::new(String::from("1"))]);
        assert!(result.is_ok_and(|x| x == true));


        let result = default_functions[&SingleInputFunction::Lower](Box::new(String::from("0")), vec![Box::new(String::from("1"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::LowerEq](Box::new(String::from("0")), vec![Box::new(String::from("1"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::Between](Box::new(String::from("2")), vec![Box::new(String::from("1")), Box::new(String::from("3"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::BetweenEq](Box::new(String::from("1")), vec![Box::new(String::from("1")), Box::new(String::from("3"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::NotBetween](Box::new(String::from("2")), vec![Box::new(String::from("1")), Box::new(String::from("3"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::NotBetweenEq](Box::new(String::from("1")), vec![Box::new(String::from("1")), Box::new(String::from("3"))]);
        assert!(result.is_ok_and(|x| x == false));

        let result = default_functions[&SingleInputFunction::EqualIgnoreCase](Box::new(String::from("Ankit")), vec![Box::new(String::from("ANKIT"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::EqualAnyIgnoreCase](Box::new(String::from("Ankit")), vec![Box::new(String::from("SHEORAN")), Box::new(String::from("ANKIT"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::EqualAny](Box::new(String::from("Ankit")), vec![Box::new(String::from("SHEORAN")), Box::new(String::from("Ankit"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::NotEqualAny](Box::new(String::from("Ankit")), vec![Box::new(String::from("SHEORAN")), Box::new(String::from("ANKIT"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::StartsWith](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("Ankit"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::StartsWithIgnoreCase](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("ANKIT"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::EndsWith](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("Sheoran"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::EndsWithIgnoreCase](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("SHEORAN"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::Contains](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("Sheoran"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&SingleInputFunction::ContainsIgnoreCase](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("SHEORAN"))]);
        assert!(result.is_ok_and(|x| x == true));
    }
}