// Code that follows along with docs/syntax.txt (all of Rust's syntax, as quickly as possible).
//
// Run it with:  cargo run --bin syntax
//
// The video has one section per topic, so this program has one module per section:
//   variables.rs       variables, mutability, constants, statics, type aliases, casting
//   types.rs           scalar and compound data types
//   functions.rs       functions, statements and expressions
//   control_flow.rs    if, loop, while, for, ranges, loop labels
//   ownership.rs       ownership rules, Copy, borrowing, references, slices
//   structs.rs         structs and impl blocks
//   enums.rs           enums and pattern matching
//   errors.rs          Option, Result, the ? operator, panic, unwrap helpers
//   collections.rs     Vec, String, HashMap
//   traits.rs          generics, traits, trait bounds, derive, trait objects
//   lifetimes.rs       lifetime annotations, elision, 'static
//   closures.rs        closures and iterators
//   smart_pointers.rs  Box, Deref, Rc, Arc, RefCell, Mutex, RwLock, Drop
//   concurrency.rs     threads, channels, async
//   advanced.rs        modules, attributes, macros, unsafe
// The last section, cargo commands, is a comment at the bottom of this file.
// Each module has a `pub fn run()` that main() calls in order.

mod advanced; // declare each module; the compiler reads its code from the .rs file with that name
mod closures;
mod collections;
mod concurrency;
mod control_flow;
mod enums;
mod errors;
mod functions;
mod lifetimes;
mod ownership;
mod smart_pointers;
mod structs;
mod traits;
mod types;
mod variables;

fn main() {
    // Each heading starts with \n, a new line, to leave a blank line before it.
    println!("--- Variables, mutability, constants, type casting ---");
    variables::run(); // `module::item` reaches an item inside a module

    println!("\n--- Scalar and compound data types ---");
    types::run();

    println!("\n--- Functions, statements, expressions ---");
    functions::run();

    println!("\n--- Control flow ---");
    control_flow::run();

    println!("\n--- Ownership, Copy, borrowing, slices ---");
    ownership::run();

    println!("\n--- Structs and impl blocks ---");
    structs::run();

    println!("\n--- Enums and pattern matching ---");
    enums::run();

    println!("\n--- Error handling: Option and Result ---");
    errors::run();

    println!("\n--- Collections: Vec, String, HashMap ---");
    collections::run();

    println!("\n--- Generics and traits ---");
    traits::run();

    println!("\n--- Lifetimes ---");
    lifetimes::run();

    println!("\n--- Closures and iterators ---");
    closures::run();

    println!("\n--- Smart pointers, Deref, Drop ---");
    smart_pointers::run();

    println!("\n--- Concurrency and async ---");
    concurrency::run();

    println!("\n--- Modules, attributes, macros, unsafe ---");
    advanced::run();
}

// ---------------------------------------------------------------------------
// Essential cargo workflow commands
// ---------------------------------------------------------------------------
//
//     cargo new <name>   create a new project: Cargo.toml and src/main.rs
//     cargo build        compile to target/debug/ (add --release for an optimized build)
//     cargo run          build and run; `--bin syntax` picks which program in this repo
//     cargo check        type-check without producing a program, much faster than build
//     cargo test         build and run every #[test] function
//     cargo fmt          reformat the code to the standard style
//     cargo clippy       lint: point out code that works but could be simpler or more correct
