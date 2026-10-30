// Modules, attributes, macros, and unsafe Rust.

// ---------------------------------------------------------------------------
// Modules and visibility
// ---------------------------------------------------------------------------

// `mod name { }` declares a module inline. (`mod name;` loads it from name.rs instead,
// which is how main.rs loads this file.) Everything inside is private by default.
mod shapes {
    pub mod circle {
        // `pub` makes it visible to everyone who can see the circle module.
        pub fn area(r: f64) -> f64 {
            super::PI_ISH * r * r // `super` is the parent module, here shapes
        }

        // `pub(crate)`: visible anywhere in this crate (this program), but not to other crates.
        pub(crate) fn describe() -> &'static str {
            "a circle"
        }

        // `pub(super)`: visible only to the parent module, shapes.
        pub(super) fn secret() -> &'static str {
            "only shapes can call this"
        }
    }

    const PI_ISH: f64 = 3.14; // private: only code inside shapes (and its children) can use it

    pub fn reveal() -> &'static str {
        circle::secret() // allowed: we're in the parent module
    }
}

// `use` imports a path so it can be used by a shorter name.
use shapes::circle;

// ---------------------------------------------------------------------------
// Attributes
// ---------------------------------------------------------------------------

// #[cfg(...)] compiles an item only when the condition holds. Exactly one of these two
// functions exists in the built program.
#[cfg(target_os = "windows")]
fn platform() -> &'static str {
    "windows"
}
#[cfg(not(target_os = "windows"))]
fn platform() -> &'static str {
    "not windows"
}

// #[inline] hints that the compiler should paste this function's body into its callers.
#[inline]
fn double(x: i32) -> i32 {
    x * 2
}

// #[allow(...)] turns a lint warning off for one item; #[deny(...)] turns it into an error.
#[allow(dead_code)] // no warning even though nothing calls this
fn never_called() {}

// ---------------------------------------------------------------------------
// Declarative macros
// ---------------------------------------------------------------------------

// macro_rules! defines a macro: code that writes code at compile time.
// Each rule is (pattern) => { expansion }. `$x:expr` matches any expression.
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

// `$( ... ),*` matches a comma-separated list, repeated zero or more times.
macro_rules! sum_all {
    ($($x:expr),*) => {
        0 $(+ $x)* // expands to 0 + a + b + c ...
    };
}

// ---------------------------------------------------------------------------
// Unsafe
// ---------------------------------------------------------------------------

// Declares a function from the C standard library (FFI: foreign function interface).
// Since edition 2024 the block itself is marked `unsafe`, because Rust can't check that
// the signature we wrote matches the real C function.
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn run() {
    println!("circle area = {:.2}", circle::area(2.0)); // {:.2}: two decimal places
    println!("{}, {}", circle::describe(), shapes::reveal());
    // circle::secret(); // error[E0603]: function `secret` is private

    println!("platform = {}, double(21) = {}", platform(), double(21));

    println!("square!(7) = {}", square!(7)); // expands to 7 * 7
    println!("sum_all!(1, 2, 3) = {}", sum_all!(1, 2, 3)); // expands to 0 + 1 + 2 + 3

    let x = 42;
    let ptr = &x as *const i32; // creating a raw pointer is safe
    // An `unsafe` block lets you do what the compiler can't check. You promise it's correct.
    unsafe {
        println!("*ptr = {}", *ptr); // dereferencing a raw pointer needs unsafe
        println!("abs(-3) = {}", abs(-3)); // calling a C function needs unsafe
    }
}
