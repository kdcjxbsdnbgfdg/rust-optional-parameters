#![allow(unused_parens, non_snake_case, unused_variables, unused)]
// -- --nocapture for print

use optional_params::default_params;


#[default_params(rhs = 10)]
fn add(lhs: i32, rhs: i32) -> i32 {
    return rhs + lhs;
}
#[default_params(lhs = 10, rhs = 10)]
fn add2(lhs: i32, rhs: i32) -> i32 {
    return rhs + lhs;
}

#[default_params(lhs = 1.0, rhs = 2.0)]
fn div(lhs: f32, rhs: f32) -> f32 {
    return lhs / rhs;
}

#[default_params(b = -2, c = 4)]
fn threeParams(a: i32, b: i32, c: i32) -> i32 {
    return (a + b) - c;
}


#[test]
fn testAdd() {
    assert_eq!(add(20, 10), add!(20));
    assert_eq!(add(20, 40), add!(20, .rhs = 40));
}

#[test]
fn testOutOfOrder() {
    assert_eq!(add2(6, 7), add2!(.rhs = 7, .lhs = 6));
    assert_eq!(div(6.0, 7.0), div!(.rhs = 7.0, .lhs = 6.0));
}

#[test]
fn testNoParams() {
    assert_eq!(add2(10, 10), add2!());
    assert_eq!(div(1.0, 2.0), div!());
}

#[test]
fn testOverrideParams() {
    assert_eq!(add2(1, 10), add2!(.lhs = 1));
    assert_eq!(add2(10, 1), add2!(.rhs = 1));
}

#[test]
fn testThreeParams() {
    assert_eq!(threeParams(1, 2, 3), threeParams!(1, .b = 2, .c = 3));
    assert_eq!(threeParams(1, 2, 3), threeParams!(1, .c = 3, .b = 2));
}
