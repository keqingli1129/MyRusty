// Functions, statements, and expressions.

// `fn` declares a function. Names use snake_case.
// Every parameter needs a type annotation; the return type comes after `->`.
fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon: this is an expression, and the last one in the body is returned
}

// `return` exits early with a value.
fn absolute(n: i32) -> i32 {
    if n < 0 {
        return -n; // leave the function right here
    }
    n // otherwise fall through to the final expression
}

// A function without `->` returns `()`, the unit type, which means "nothing".
fn greet(name: &str) {
    println!("Hello, {name}!");
}

pub fn run() {
    println!("add(2, 3) = {}", add(2, 3));
    println!("absolute(-7) = {}", absolute(-7));
    greet("Rust");

    // A statement performs an action and ends with `;`. It produces no value.
    let x = 5; // `let` is a statement

    // An expression evaluates to a value. A `{ }` block is an expression too:
    // its value is its last expression, the one without a semicolon.
    let y = {
        let doubled = x * 2; // a statement inside the block
        doubled + 1 // the block's value: 11
    };
    println!("y = {y}");

    // Adding a semicolon turns an expression into a statement, and the block becomes `()`.
    // The compiler warns that the result of x + 1 is unused; #[allow] silences it for this demo.
    #[allow(unused_must_use)]
    let unit = {
        x + 1; // the value is thrown away
    };
    println!("unit = {unit:?}"); // prints ()
}
