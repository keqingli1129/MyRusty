// Control flow: if, loop, while, for, ranges, and loop labels.

pub fn run() {
    // ---------------------------------------------------------------------------
    // if / else if / else
    // ---------------------------------------------------------------------------

    let number = 7;
    // No parentheses around the condition; the curly braces are required.
    if number % 2 == 0 {
        println!("{number} is even");
    } else if number % 3 == 0 {
        println!("{number} is divisible by 3");
    } else {
        println!("{number} is something else");
    }
    // The condition must be a bool. `if number { }` is an error: Rust doesn't treat 0 as false.

    // `if` is an expression, so its result can be assigned. Both branches must have the same type.
    let parity = if number % 2 == 0 { "even" } else { "odd" };
    println!("parity = {parity}");

    // ---------------------------------------------------------------------------
    // loop, break, continue
    // ---------------------------------------------------------------------------

    let mut counter = 0;
    // `loop` repeats forever until something breaks out of it.
    let result = loop {
        counter += 1;
        if counter == 2 {
            continue; // skip the rest of this iteration and start the next one
        }
        if counter == 5 {
            break counter * 10; // exit the loop; the value after `break` is the loop's value
        }
    };
    println!("result = {result}"); // 50

    // A label starts with a single quote. `break 'outer` leaves the outer loop, not just the inner.
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i + j == 3 {
                println!("breaking out at i = {i}, j = {j}");
                break 'outer; // without the label this would only end the inner loop
            }
        }
    }

    // ---------------------------------------------------------------------------
    // while
    // ---------------------------------------------------------------------------

    let mut n = 3;
    // `while` runs as long as its condition is true.
    while n > 0 {
        print!("{n}... "); // print! does not add a new line
        n -= 1;
    }
    println!("liftoff!");

    // ---------------------------------------------------------------------------
    // for and ranges
    // ---------------------------------------------------------------------------

    let fruits = ["apple", "banana", "cherry"];
    // `for` walks over anything iterable: here, the elements of an array.
    for fruit in fruits {
        print!("{fruit} ");
    }
    println!();

    // `a..b` is a range that EXCLUDES b: 1, 2, 3, 4.
    for i in 1..5 {
        print!("{i} ");
    }
    println!();
    // `a..=b` INCLUDES b: 1, 2, 3, 4, 5.
    for i in 1..=5 {
        print!("{i} ");
    }
    println!();
    // .rev() runs a range backwards.
    for i in (1..=3).rev() {
        print!("{i} ");
    }
    println!();
}
