use std::collections::HashMap;
use std::error::Error;
use std::any::Any;
use std::io;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum FunctionName {
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

fn default() -> HashMap<FunctionName, Function> {
    let mut map: HashMap<FunctionName, Function>  = HashMap::new();
    default_general_functions(&mut map);
    default_numeric_function(&mut map);
    default_string_function(&mut map);
    map
}

fn default_general_functions(m : &mut HashMap<FunctionName, Function>) -> &mut HashMap<FunctionName, Function> {
   // let mut m = HashMap::new();
    
    m.insert(FunctionName::Empty, Box::new(|input: Box<dyn Any>, _args: Vec<Box<dyn Any>>| {
        match input.downcast_ref::<String>() {
            Some(string) => Ok(string.is_empty()),
            None => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Input is not a string")) as Box<dyn Error>),
        }
    })as Function);

    m.insert(FunctionName::NonEmpty, Box::new(|input: Box<dyn Any>, _args: Vec<Box<dyn Any>>| {
        match input.downcast_ref::<String>() {
            Some(string) => Ok(string.is_empty() == false),
            None => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Input is not a string")) as Box<dyn Error>),
            
        }
    }));

    m.insert(FunctionName::Equal, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
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

fn default_numeric_function(m: &mut HashMap<FunctionName, Function>) -> &mut HashMap<FunctionName, Function>{

    m.insert(FunctionName::Greater, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(FunctionName::Greater, &*input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num > args_no[0]),
            Err(e) => Err(e)
        }
    }));

    m.insert(FunctionName::GreaterEq, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(FunctionName::GreaterEq, &*input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num >= args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(FunctionName::Lower, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(FunctionName::Lower, &*input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num < args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(FunctionName::LowerEq, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(FunctionName::LowerEq, &*input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num <= args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(FunctionName::Between, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(FunctionName::Between, &*input, &args_refs, 2) {
            Ok((num, args_no)) =>  Ok(num > args_no[0] && num < args_no[1]),
            Err(e) => Err(e)
            
        }
    }));
    
    m.insert(FunctionName::BetweenEq, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(FunctionName::BetweenEq, &*input, &args_refs, 2) {
            Ok((num, args_no)) =>  Ok(num >= args_no[0] && num <= args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(FunctionName::NotBetween, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(FunctionName::Between, &*input, &args_refs, 2) {
            Ok((num, args_no)) =>  Ok(num <= args_no[0] || num >= args_no[0]),
            Err(e) => Err(e)
            
        }
    }));

    m.insert(FunctionName::NotBetweenEq, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(FunctionName::NotBetweenEq, &*input, &args_refs, 2) {
            Ok((num, args_no)) =>  Ok(num > args_no[0] && num < args_no[0]),
            Err(e) => Err(e)
            
        }
    }));


    m

}

fn parse_numeric( function_name: FunctionName ,
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



fn default_string_function(m: &mut HashMap<FunctionName, Function>)  -> &mut HashMap<FunctionName, Function> {
    m.insert(FunctionName::EqualIgnoreCase, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EqualIgnoreCase: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.eq_ignore_ascii_case(arg_str))
    }));

    m.insert(FunctionName::EqualAnyIgnoreCase, Box::new(|input, args| {
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

    m.insert(FunctionName::EqualAny, Box::new(|input, args| {
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

    m.insert(FunctionName::NotEqualAny, Box::new(|input, args| {
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

    m.insert(FunctionName::StartsWith, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("StartsWith: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.starts_with(arg_str))
    }));

    m.insert(FunctionName::StartsWithIgnoreCase, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("StartsWith: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.to_lowercase().starts_with(&arg_str.to_lowercase()))
    }));

    m.insert(FunctionName::EndsWith, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EndsWith: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.ends_with(arg_str))
    }));

    m.insert(FunctionName::EndsWithIgnoreCase, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EndsWithIgnoreCase: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.to_lowercase().ends_with(&arg_str.to_lowercase()))
    }));

    m.insert(FunctionName::Contains, Box::new(|input, args| {
        if args.len() != 1 {
            return Err("EndsWithIgnoreCase: needs one argument".into());
        }
        let input_str = input.downcast_ref::<String>().ok_or("Input is not a string")?;
        let arg_str = args[0].downcast_ref::<String>().ok_or("Argument is not a string")?;
        Ok(input_str.contains(arg_str))
    }));
    m.insert(FunctionName::ContainsIgnoreCase, Box::new(|input, args| {
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

        let result = default_functions[&FunctionName::Empty](Box::new(String::from("")), vec![]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::NonEmpty](Box::new(String::from("ankit")), vec![]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::Equal](Box::new(String::from("ankit")), vec![Box::new(String::from("ankit"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::Equal](Box::new(String::from("ankit")), vec![Box::new(String::from("anki"))]);
        assert!(result.is_ok_and(|x| x == false));
    
        let result = default_functions[&FunctionName::Greater](Box::new(String::from("1")), vec![Box::new(String::from("0"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::GreaterEq](Box::new(String::from("1")), vec![Box::new(String::from("1"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::Lower](Box::new(String::from("0")), vec![Box::new(String::from("1"))]);
        assert!(result.is_ok_and(|x| x == true));


        let result = default_functions[&FunctionName::Lower](Box::new(String::from("0")), vec![Box::new(String::from("1"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::LowerEq](Box::new(String::from("0")), vec![Box::new(String::from("1"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::Between](Box::new(String::from("2")), vec![Box::new(String::from("1")), Box::new(String::from("3"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::BetweenEq](Box::new(String::from("1")), vec![Box::new(String::from("1")), Box::new(String::from("3"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::NotBetween](Box::new(String::from("2")), vec![Box::new(String::from("1")), Box::new(String::from("3"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::NotBetweenEq](Box::new(String::from("1")), vec![Box::new(String::from("1")), Box::new(String::from("3"))]);
        assert!(result.is_ok_and(|x| x == false));

        let result = default_functions[&FunctionName::EqualIgnoreCase](Box::new(String::from("Ankit")), vec![Box::new(String::from("ANKIT"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::EqualAnyIgnoreCase](Box::new(String::from("Ankit")), vec![Box::new(String::from("SHEORAN")), Box::new(String::from("ANKIT"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::EqualAny](Box::new(String::from("Ankit")), vec![Box::new(String::from("SHEORAN")), Box::new(String::from("Ankit"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::NotEqualAny](Box::new(String::from("Ankit")), vec![Box::new(String::from("SHEORAN")), Box::new(String::from("ANKIT"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::StartsWith](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("Ankit"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::StartsWithIgnoreCase](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("ANKIT"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::EndsWith](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("Sheoran"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::EndsWithIgnoreCase](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("SHEORAN"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::Contains](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("Sheoran"))]);
        assert!(result.is_ok_and(|x| x == true));

        let result = default_functions[&FunctionName::ContainsIgnoreCase](Box::new(String::from("AnkitSheoran")), vec![Box::new(String::from("SHEORAN"))]);
        assert!(result.is_ok_and(|x| x == true));
    }
}