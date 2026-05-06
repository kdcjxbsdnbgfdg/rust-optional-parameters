#![allow(unused_parens, non_snake_case, unused_variables)]
// -- --nocapture for print

use optional_params::default_params;

//fn testfun(param: u32,){}

//macro_rules! testFunction {
//    ($($meow:expr),*) => {
//       testFunction($($meow),*)
//    };
//    ($($meow:expr),*, $(.$paramName:ident = $paramValue:expr),*) => {
//       testFunction($($meow),*)
//    };
//}

#[default_params(arg2 = "hello world")]
fn testFunction(arg1: u32, arg2: &str) {
    for _ in (0..arg1) {
        println!("{:?}", arg2);
    }
}

#[default_params(rhs = 10)]
fn add(lhs: i32, rhs: i32) -> i32 {
    return rhs + lhs;
}


#[test]
fn meow() {
    // regular function call
    testFunction(10, "goodbye world");
    // all default parameters
    testFunction!(10);
    // overriding a default parameter
    testFunction!(10, .arg2 = "override");
}

#[test]
fn testAdd() {
    assert_eq!(add(20, 10), add!(20));
    assert_eq!(add(20, 40), add!(20, .rhs = 40));
}
