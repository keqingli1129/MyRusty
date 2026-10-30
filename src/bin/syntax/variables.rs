// Variables, mutability, constants, and type casting (docs/syntax.txt, first section).

// `const` declares a constant. It's evaluated at compile time and copied into every place
// that uses it. Constants need a type annotation and are named in SCREAMING_SNAKE_CASE.
const MAX_POINTS: u32 = 100_000; // underscores in number literals are only for readability

// `static` declares a value with one fixed memory location for the whole run of the program.
// Use it when you need a single address (for example a big table); otherwise prefer const.
static GREETING: &str = "hello";

// `type` creates an alias: a second name for an existing type. It does not create a new type,
// so a Kilometers and an i32 can be mixed freely.
type Kilometers = i32;

pub fn run() {
    // `let` declares a variable. Variables are immutable by default.
    let x = 5; // the type, i32, is inferred from the literal
    println!("x = {x}");
    // x = 6; // error[E0384]: cannot assign twice to immutable variable `x`

    // `mut` makes a variable mutable, so it can be assigned again.
    let mut y = 5;
    y += 1; // allowed because y is mut
    println!("y = {y}");

    // A type annotation goes after the name with a colon. It's optional when Rust can infer it.
    let z: f64 = 2.5;
    println!("z = {z}");

    // Shadowing: a new `let` with the same name creates a new variable that hides the old one.
    // Unlike `mut`, the new variable can even have a different type.
    let spaces = "   "; // a &str
    let spaces = spaces.len(); // now a usize, the number of characters
    println!("spaces = {spaces}");

    // Constants and statics are used like any other value.
    println!("MAX_POINTS = {MAX_POINTS}, GREETING = {GREETING}");

    // The alias is interchangeable with i32.
    let distance: Kilometers = 42;
    let total: i32 = distance + 8; // adding a Kilometers to an i32 is fine
    println!("total = {total}");

    // `as` casts between primitive types. Rust never converts number types implicitly.
    let n: i32 = 7;
    let f = n as f64 / 2.0; // i32 -> f64, so the division keeps the fraction: 3.5
    let truncated = 3.99_f64 as i32; // f64 -> i32 drops the fraction: 3
    let byte = 300_i32 as u8; // a too-big integer wraps around: 300 - 256 = 44
    println!("f = {f}, truncated = {truncated}, byte = {byte}");

    // `as` can also cast a reference to a raw pointer. Making a raw pointer is safe;
    // only reading through one needs `unsafe` (see advanced.rs).
    let ptr = &n as *const i32;
    println!("ptr = {ptr:?}"); // {:?} prints the address
}
