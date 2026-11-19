// Guessing game, step 1: prompt the user and read a number from the keyboard.
//
// Run it with:  cargo run --bin guessgame

use std::io::{self, Write}; // `io` for stdin/stdout; the `Write` trait gives us `flush()`

// Ask until the user types a whole number, then return it.
fn read_number(prompt: &str) -> i32 {
    loop {
        print!("{prompt}"); // print! does not add a newline, so the cursor stays on this line
        io::stdout().flush().expect("failed to flush stdout"); // print! is buffered; flush shows it now

        let mut input = String::new(); // read_line appends to a String we own
        io::stdin()
            .read_line(&mut input) // reads up to and including the Enter key ('\n')
            .expect("failed to read line");

        // trim() drops the trailing newline and spaces; parse() turns text into a number.
        // parse returns a Result, so match on it: Ok holds the number, Err means bad input.
        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("'{}' is not a whole number, try again.", input.trim()),
        }
    }
}

fn main() {
    let guess = read_number("Enter a number: ");
    println!("You entered {guess}.");
}
