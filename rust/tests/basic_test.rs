// Basic Test, this is meant for setting up
use googletest::prelude::*;
use adventOfCode::tutorial;

#[test]
fn basic_test() {
    let args = 42;
    let results = tutorial::tutorial_function(args); 
    assert_that!(results, eq(84));
}

#[googletest::test]
fn basic_test2() {
    let results = tutorial::tutorial_function(42);
    verify_that!(results, eq(84));
}