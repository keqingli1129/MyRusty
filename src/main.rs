// Code that follows along with docs/intro.txt (Rust Curious, lesson 2: basic syntax).
// Each section of the transcript has its own function, and main() runs them in order.

// ---------------------------------------------------------------------------
// Hello World
// ---------------------------------------------------------------------------

// `fn` introduces a function; this one takes no arguments and returns nothing.
fn hello_world() {
    // `println!` ends in `!`, so it is a macro, not a function.
    // It takes a variable number of arguments and checks the format string at compile time.
    println!("Hello, world!");
} // Curly braces delimit the body; whitespace has no meaning.

// ---------------------------------------------------------------------------
// Variables - let
// ---------------------------------------------------------------------------

fn variables_let() {
    // `let` declares x; the text after the colon is its type (u32 = unsigned 32-bit int).
    let x: u32 = 10;
    // `{}` is a placeholder that gets replaced by the value of x at runtime.
    println!("x is {}", x);

    // Here there is no type annotation: the literal `10u32` carries its own type,
    let y = 10u32;
    // so the compiler infers that y must be a u32.
    println!("y is {}", y);
}

// ---------------------------------------------------------------------------
// Mutable variables - let mut
// ---------------------------------------------------------------------------

fn mutable_variables() {
    // Variables are immutable by default; `mut` makes this one writable.
    // No type is given and 10 is a plain integer, so the default integer type i32 is used.
    let mut x = 10;
    // Print the starting value.
    println!("x starts as {}", x);
    // Assigning again is allowed only because x was declared with `mut`.
    x = 20;
    // Print the new value.
    println!("x is now {}", x);
    // Without `mut`, the assignment above would be a compile error:
    //     let x = 10;
    //     x = 20; // error[E0384]: cannot assign twice to immutable variable `x`
}

// ---------------------------------------------------------------------------
// Scalar types
// ---------------------------------------------------------------------------

fn scalar_types() {
    // Unsigned integers ("u"): 8, 16, 32, 64 and 128 bits wide.
    let a: u8 = 255; // u8 max value
    let b: u16 = 65_535; // underscores are allowed in numbers for readability
    let c: u32 = 4_000_000_000; // too big for i32, fits in u32
    let d: u64 = 18_000_000_000_000_000_000; // just under u64::MAX
    let e: u128 = 1 << 100; // 2 to the power of 100
    // Print all the unsigned values on one line.
    println!("unsigned: {} {} {} {} {}", a, b, c, d, e);

    // Signed integers ("i"): same widths, can hold negative values.
    let f: i8 = -128; // i8 min value
    let g: i16 = -32_768; // i16 min value
    let h: i32 = -2_000_000_000; // i32 is the default integer type
    let i: i64 = -9_000_000_000_000_000_000; // just above i64::MIN
    let j: i128 = -(1 << 100); // negative 2 to the power of 100
    // Print all the signed values on one line.
    println!("signed: {} {} {} {} {}", f, g, h, i, j);

    // usize/isize are as wide as a pointer (32 or 64 bits depending on the target).
    let length: usize = 3; // usize is the type used for indexes and lengths
    let offset: isize = -1; // isize is the signed equivalent, used far less often
    // Print the sizes, and how many bytes a usize is on this machine.
    println!(
        "usize {} isize {} (usize is {} bytes here)",
        length,
        offset,
        std::mem::size_of::<usize>()
    );

    // Floating point: f32 is like C's float, f64 is like C's double (IEEE 754).
    let k: f32 = 3.14; // 32-bit float
    let avogadro: f64 = 6.022e23; // scientific notation: 6.022 times ten to the 23
    let big: f64 = 6.022e28; // the transcript's example: 6.022 times ten to the 28
    // Print the floats.
    println!("floats: {} {} {}", k, avogadro, big);

    // bool is one byte and only ever holds `true` or `false` (both are keywords).
    let yes: bool = true; // true value
    let no = false; // type inferred as bool
    // Print the booleans.
    println!("bools: {} {}", yes, no);

    // char is a 32-bit "unicode scalar value"; char literals use single quotes.
    let letter: char = 'R'; // plain ASCII letter
    let thumbs = '👍'; // a single-codepoint emoji fits in a char
    // An emoji with a skin tone is several codepoints, so it must be a string, not a char:
    let thumbs_dark = "👍🏿"; // double quotes = string
    // Print the chars and the string, plus the size of a char in bytes (always 4).
    println!(
        "chars: {} {} string: {} (char is {} bytes)",
        letter,
        thumbs,
        thumbs_dark,
        std::mem::size_of::<char>()
    );
}

// ---------------------------------------------------------------------------
// Casting between scalar types - as
// ---------------------------------------------------------------------------

fn casting() {
    // Rust never converts between scalar types implicitly.
    let c = 'A'; // c is a char
    // `c as u32` explicitly converts the char to its integer value.
    println!("{} as u32 is {}", c, c as u32);
    // `{:x}` in the placeholder means "format as hexadecimal".
    println!("{} as hex is {:x}", c, c as u32);

    // Going the other way, u32 -> char, is NOT allowed with `as`,
    // because not every u32 value is a valid char:
    //     let bad = 65u32 as char; // error[E0604]: only `u8` can be cast as `char`, not `u32`
    // A u8 is always a valid char, so that cast is allowed.
    let from_byte = 65u8 as char;
    // Print the char made from the byte.
    println!("65u8 as char is {}", from_byte);
}

// ---------------------------------------------------------------------------
// Block expressions
// ---------------------------------------------------------------------------

fn block_expressions() {
    // The right-hand side of `=` can be a whole block of code in curly braces.
    let x = {
        let a = 10; // temporary variable that only exists inside the block
        let b = 20; // another temporary
        a + b // final expression, NO semicolon: this is the value of the block
    }; // this semicolon ends the `let` statement
    // x gets the value 30.
    println!("x from block is {}", x);
}

// ---------------------------------------------------------------------------
// Function return types
// ---------------------------------------------------------------------------

// Takes a u32 named x and returns a u32. The return type comes after `->`.
// Parameter and return types must always be written out explicitly.
fn triple(x: u32) -> u32 {
    x * 3 // final expression with no semicolon becomes the return value
}

// ---------------------------------------------------------------------------
// Mismatched types error
// ---------------------------------------------------------------------------
//
// Adding a semicolon after the final expression means the body produces
// "unit", written (), which is the type for "nothing". That doesn't compile:
//
//     fn triple(x: u32) -> u32 {
//         x * 3;
//     }
//
//     error[E0308]: mismatched types
//      expected `u32`, found `()`
//      note: implicitly returns `()` as its body has no tail or `return` expression
//      help: remove this semicolon to return this value
//
// Read the compiler errors: they're usually helpful.

fn function_return_types() {
    // Call the function and print the result: 7 * 3 = 21.
    println!("triple(7) is {}", triple(7));
}

// ---------------------------------------------------------------------------
// Conditionals - if-else
// ---------------------------------------------------------------------------

fn conditionals() {
    let x = 5; // value to test
    // No parentheses needed around the condition, but the braces are required.
    if x > 3 {
        // runs when the condition is true
        println!("{} is greater than 3", x);
    }

    // `else` runs when the condition is false.
    if x > 10 {
        println!("{} is greater than 10", x); // not printed
    } else {
        println!("{} is not greater than 10", x); // printed
    }

    // if-else is an expression: its value is whichever branch ran.
    // An `else` is required so there's a value in every case.
    // This replaces C's ternary operator `cond ? a : b`.
    let size = if x > 3 { "big" } else { "small" };
    // Print the chosen value.
    println!("{} is {}", x, size);

    // Use the max functions below.
    println!("max_verbose(4, 9) is {}", max_verbose(4, 9));
    println!("max(4, 9) is {}", max(4, 9));
}

// First version: stores the if-else result in a temporary variable.
fn max_verbose(a: i32, b: i32) -> i32 {
    let max = if a > b { a } else { b }; // pick the larger value
    max // return the temporary
}

// Simpler version: the if-else is the final expression, so it is the return value.
fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// ---------------------------------------------------------------------------
// Early returns
// ---------------------------------------------------------------------------

// Average of `total` over `count` items, returning 0 if count is zero.
fn average(total: u32, count: u32) -> u32 {
    if count == 0 {
        // `return` leaves the function immediately, skipping the division below.
        return 0;
    }
    total / count // normal path: the final expression is the return value
}

fn early_returns() {
    // Normal case: 100 / 4 = 25.
    println!("average(100, 4) is {}", average(100, 4));
    // Edge case: count is zero, so we hit the early return instead of dividing by zero.
    println!("average(100, 0) is {}", average(100, 0));
}

// ---------------------------------------------------------------------------
// Match expressions, patterns
// ---------------------------------------------------------------------------

// Returns a description of a character.
// `&'static str` is a reference to a string that is valid everywhere,
// which is always true for string literals.
fn describe(c: char) -> &'static str {
    // `match` tests c against each pattern in order; the first match wins.
    match c {
        '👍' => "like", // pattern on the left, expression on the right of `=>`
        '🔔' => "subscribe", // a single specific char
        '[' | ']' => "bracket", // `|` matches either of the two chars
        '5' => "five", // checked before the digit range, so '5' gives "five"
        '0'..='9' => "digit", // `..=` is an inclusive range: '0' to '9'
        'a'..='z' | 'A'..='Z' => "letter", // several ranges combined in one pattern
        _ if (c as u32) < 128 => "other ascii", // `_` matches anything; `if` adds a guard
        _ => "something else", // catch-all so the match is exhaustive
    }
}

fn match_expressions() {
    // Try some chars that hit different arms.
    for c in ['👍', '🔔', '[', '5', '7', 'q', 'Q', '#', 'é'] {
        // Print each char and the arm it matched.
        println!("describe({:?}) is {:?}", c, describe(c));
    }
}

// ---------------------------------------------------------------------------
// Loops - loop, break
// ---------------------------------------------------------------------------

// Small pseudo-random number generator (xorshift), so we don't need an extra crate.
// It takes the previous state and returns the next state.
fn next_random(state: u32) -> u32 {
    let mut x = state; // copy the state so we can modify it
    x ^= x << 13; // mix the bits: shift left and XOR
    x ^= x >> 17; // shift right and XOR
    x ^= x << 5; // shift left and XOR again
    x // the new state is also the random value
}

fn loop_break() {
    let mut seed = 12345; // starting state for the random generator

    // `loop` is an infinite loop; `break` exits it.
    loop {
        seed = next_random(seed); // advance the generator
        let x = seed % 256; // random value from 0 to 255
        println!("random x = {}", x); // print every value we try
        if x > 200 {
            break; // leave the loop once x > 200
        }
        // `continue` also exists, like in C: it jumps to the next iteration.
    }

    // `loop` can also be an expression: `break value` gives the loop that value.
    let y = loop {
        seed = next_random(seed); // advance the generator
        let x = seed % 256; // random value from 0 to 255
        if x > 200 {
            break x; // the loop expression evaluates to x
        }
    }; // semicolon ends the `let` statement
    // y holds the first value that was greater than 200.
    println!("kept y = {}", y);
}

// ---------------------------------------------------------------------------
// Loops - while
// ---------------------------------------------------------------------------

fn while_loop() {
    let mut x = 5; // countdown starting value
    // Repeat while the condition is true; no parens, braces required, condition must be a bool.
    while x > 0 {
        println!("while x = {}", x); // print the current value
        x -= 1; // count down so the loop eventually stops
    }
}

// ---------------------------------------------------------------------------
// Loops - for, ranges .. and ..=
// ---------------------------------------------------------------------------

fn for_loops() {
    // `0..10` is a half-open range: it includes 0 but not 10, so this prints 0 through 9.
    for x in 0..10 {
        print!("{} ", x); // print! (no "ln") stays on the same line
    }
    println!(); // finish the line

    // `0..=10` is a closed (inclusive) range, so this prints 0 through 10.
    for x in 0..=10 {
        print!("{} ", x); // x only exists inside the loop body
    }
    println!(); // finish the line
    // Ranges are one kind of iterator; `for` works with any iterator.
}

// ---------------------------------------------------------------------------
// Practice exercise: factorial
// ---------------------------------------------------------------------------

// The version the exercise starts with: recursive.
// It uses stack memory proportional to n, and Rust doesn't guarantee tail call optimisation.
#[allow(dead_code)] // only called from the tests, so silence the "never used" warning
fn factorial_recursive(n: u64) -> u64 {
    if n > 1 {
        n * factorial_recursive(n - 1) // n! = n * (n-1)!
    } else {
        1 // 0! and 1! are both 1
    }
}

// The exercise solution: iterative, using a loop and a mutable variable.
// factorial(5) = 1 * 2 * 3 * 4 * 5
fn factorial(n: u64) -> u64 {
    let mut result = 1; // start at 1 (empty product), which also covers 0! and 1!
    for i in 1..=n {
        // i runs from 1 to n inclusive
        result *= i; // multiply the running product by i
    }
    result // return the product
}

fn factorial_exercise() {
    // Print factorials from 0 to 20. 20! is the largest that fits in a u64;
    // changing this to 0..=25 makes it overflow and panic in a debug build.
    for n in 0..=20 {
        println!("factorial({}) = {}", n, factorial(n));
    }
}

// ---------------------------------------------------------------------------
// main: run every section in transcript order
// ---------------------------------------------------------------------------

// Execution starts here.
fn main() {
    println!("--- Hello World ---"); // section header
    hello_world(); // run the section

    println!("\n--- Variables - let ---");
    variables_let();

    println!("\n--- Mutable variables - let mut ---");
    mutable_variables();

    println!("\n--- Scalar types ---");
    scalar_types();

    println!("\n--- Casting - as ---");
    casting();

    println!("\n--- Block expressions ---");
    block_expressions();

    println!("\n--- Function return types ---");
    function_return_types();

    println!("\n--- Conditionals - if-else ---");
    conditionals();

    println!("\n--- Early returns ---");
    early_returns();

    println!("\n--- Match expressions ---");
    match_expressions();

    println!("\n--- Loops - loop, break ---");
    loop_break();

    println!("\n--- Loops - while ---");
    while_loop();

    println!("\n--- Loops - for, ranges ---");
    for_loops();

    println!("\n--- Practice exercise: factorial ---");
    factorial_exercise();
}

// ---------------------------------------------------------------------------
// Unit test functions - #[test]
// ---------------------------------------------------------------------------

// `#[test]` is an attribute that marks the next function as a unit test.
// Tests are left out of a normal build and only compiled in by `cargo test`.
#[test]
fn test_factorial() {
    // assert_eq! panics (failing the test) if the two values differ.
    assert_eq!(factorial(0), 1); // 0! = 1 by definition
    assert_eq!(factorial(1), 1); // 1! = 1
    assert_eq!(factorial(2), 2); // 1*2
    assert_eq!(factorial(5), 120); // 1*2*3*4*5
    assert_eq!(factorial(20), 2_432_902_008_176_640_000); // largest that fits in a u64
}

// Check the iterative version agrees with the original recursive one.
#[test]
fn test_factorial_matches_recursive() {
    for n in 0..=20 {
        // compare both versions for every n from 0 to 20
        assert_eq!(factorial(n), factorial_recursive(n));
    }
}

// Tests for the other functions from the transcript.
#[test]
fn test_examples() {
    assert_eq!(triple(7), 21); // 7 * 3
    assert_eq!(max(4, 9), 9); // larger of two
    assert_eq!(max_verbose(9, 4), 9); // same answer from the verbose version
    assert_eq!(average(100, 4), 25); // normal division
    assert_eq!(average(100, 0), 0); // early return path
    assert_eq!(describe('5'), "five"); // specific arm beats the range below it
    assert_eq!(describe('7'), "digit"); // inclusive range arm
    assert_eq!(describe('#'), "other ascii"); // guard arm
    assert_eq!(describe('é'), "something else"); // catch-all arm
}
