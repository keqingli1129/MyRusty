// Error handling with Option and Result.

use std::num::ParseIntError; // the error type returned when parsing a number fails

// Rust has no null. A value that may be missing is an Option<T>: Some(value) or None.
fn find_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        // `&n` in the pattern copies the i32 out of the reference
        if n % 2 == 0 {
            return Some(n); // found one
        }
    }
    None // none found
}

// A recoverable error is a Result<T, E>: Ok(value) on success or Err(error) on failure.
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>() // parse already returns a Result, so we just return it
}

// The `?` operator: on Ok it unwraps the value, on Err it returns the error from this function.
fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = parse_number(a)?; // if this fails, add_strings returns the Err right here
    let y = parse_number(b)?;
    Ok(x + y) // both worked: wrap the sum in Ok
}

pub fn run() {
    // You have to handle both cases before you can use the value inside.
    match find_even(&[1, 3, 4, 5]) {
        Some(n) => println!("first even: {n}"),
        None => println!("no even numbers"),
    }

    match parse_number("42") {
        Ok(n) => println!("parsed {n}"),
        Err(e) => println!("error: {e}"),
    }

    println!("add_strings(\"2\", \"3\") = {:?}", add_strings("2", "3")); // Ok(5)
    println!("add_strings(\"2\", \"x\") = {:?}", add_strings("2", "x")); // Err(...)

    // Helper methods.
    let some = Some(10);
    let none: Option<i32> = None;
    println!("unwrap: {}", some.unwrap()); // gives the value; would panic on None
    println!("expect: {}", some.expect("should have a value")); // same, with your panic message
    println!("unwrap_or: {}", none.unwrap_or(0)); // gives a fallback instead of panicking
    println!(
        "unwrap_or_default: {}",
        parse_number("oops").unwrap_or_default() // the Err gives way to i32's default, 0
    );

    // panic! is for unrecoverable errors: it prints a message, unwinds the stack and stops the
    // thread. Commented out so the program keeps going (see src/bin/panic.rs for examples).
    // panic!("something went badly wrong");
    // none.unwrap(); // panics: called `Option::unwrap()` on a `None` value
}
