use std::collections::HashMap;


#[derive(Eq, PartialEq, Hash)]
pub enum ListFunction {
    All,
    AtLeast, 
    AtMost,
    Exactly, 
    NoneElement,
    AtLeastFraction,
    AtMostFraction
}

pub type ListFunctionConstraint = Box<dyn Fn(usize, usize, &[i32]) -> bool>;

fn ListFunctionConstraintsArgumentNumber(constraintName: ListFunction) -> i32 {
    match constraintName {
        ListFunction::All => 0,
        ListFunction::AtLeast => 1,
        ListFunction::AtMost => 1,
        ListFunction::Exactly => 1,
        ListFunction::NoneElement => 0,
        ListFunction::AtLeastFraction => 2,
        ListFunction::AtMostFraction => 2,
    }
}

fn default_list_function() -> HashMap<ListFunction, ListFunctionConstraint> {
    let mut m: HashMap< ListFunction, ListFunctionConstraint> = HashMap::new();
    m.insert(ListFunction::All, Box::new(|list_total, passed_total, _args| {
        passed_total == list_total
    }));

    m.insert(ListFunction::AtLeast, Box::new(|_list_total, passed_total, args| {
        passed_total >= args[0] as usize
    }));

    m.insert(ListFunction::AtMost, Box::new(|_list_total, passed_total, args| {
        passed_total <= args[0] as usize
    }));

    m.insert(ListFunction::Exactly, Box::new(|_list_total, passed_total, args| {
        passed_total == args[0] as usize
    }));

    m.insert(ListFunction::NoneElement, Box::new(|_list_total, passed_total, _args| {
        passed_total == 0
    }));

    m.insert(ListFunction::AtLeastFraction, Box::new(|list_total, passed_total, args| {
        let expected = (list_total as f64 / args[1] as f64) * args[0] as f64;
        passed_total as f64 >= expected
    }));

    m.insert(ListFunction::AtMostFraction, Box::new(|list_total, passed_total, args| {
        let expected = (list_total as f64 / args[1] as f64) * args[0] as f64;
        passed_total as f64 <= expected
    }));

    m

}