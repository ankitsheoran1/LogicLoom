use std::collections::HashMap;
use std::error::Error;
use std::any::Any;
use std::io;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum function_name {
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

fn default_general_functions(m : &mut HashMap<function_name, Function>) -> &mut HashMap<function_name, Function> {
   // let mut m = HashMap::new();
    
    m.insert(function_name::Empty, Box::new(|input: Box<dyn Any>, _args: Vec<Box<dyn Any>>| {
        match input.downcast_ref::<String>() {
            Some(string) => Ok(string.is_empty()),
            None => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Input is not a string")) as Box<dyn Error>),
        }
    })as Function);

    m.insert(function_name::NonEmpty, Box::new(|input: Box<dyn Any>, _args: Vec<Box<dyn Any>>| {
        match input.downcast_ref::<String>() {
            Some(string) => Ok(string.is_empty() == false),
            None => Err(Box::new(io::Error::new(io::ErrorKind::Other, "Input is not a string")) as Box<dyn Error>),
            
        }
    }));

    m.insert(function_name::Equal, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
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

fn default_numeric_function(m: &mut HashMap<function_name, Function>) -> &mut HashMap<function_name, Function>{

    m.insert(function_name::Greater, Box::new(|input: Box<dyn Any>, args: Vec<Box<dyn Any>>| {
        let args_refs: Vec<&dyn Any> = args.iter().map(|arg| arg.as_ref()).collect();

        match parse_numeric(function_name::Greater, &input, &args_refs, 1) {
            Ok((num, args_no)) =>  Ok(num > args_no[0]),
            Err(e) => Err(e)
        }
    }));
    m

}

fn parse_numeric( function_name: function_name ,
    input: &dyn Any,
    args: &[&dyn Any],
    required_args_count: usize) -> Result<(T, Vec<T>), Box<dyn Error>> {
        let input_ref = input.downcast_ref::<String>().ok_or_else(|| {
            format!(
                "[{:?}]: could not convert input [{:?}] to number: input is not a string",
                function_name, input
            )
        })?;
        let mut input_no;
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
                 Err(err) =>   { return format!(
                    "[{:?}]: could not convert argument [{}] [{:?}] to number: {}",
                    function_name, i, arg_ref, err
                ); }

            };
        }
        Ok((input_no, args_nr))
}



