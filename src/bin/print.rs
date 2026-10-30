// Code that follows along with docs/print.txt: ways to print to the terminal.
//
// Run it with:  cargo run --bin print
// It's in src/bin/ so cargo builds it as its own executable, named after the file.

// `#!` (with the `!`) is an inner attribute: it applies to the whole file it's in, not just
// the next item. `allow(unused)` turns off warnings about unused variables, functions etc.,
// which are common in small examples like this one. rust-analyzer in the editor follows it too.
#![allow(unused)]

// ---------------------------------------------------------------------------
// Printing a struct with Debug
// ---------------------------------------------------------------------------

// A struct for a programming language, with two String fields.
// `#[derive(Debug)]` is an attribute that tells the compiler to generate the code needed to
// print this struct with {:?} and {:#?}. Without it, printing fails to compile:
//     error[E0277]: `Lang` doesn't implement `Debug`
#[derive(Debug)]
struct Lang {
    language: String, // the language's name
    version: String,  // its version, stored as text
}

fn main() {
    // ---------------------------------------------------------------------------
    // println! with variables
    // ---------------------------------------------------------------------------

    let lang = "rust"; // a string to print

    // println! prints its arguments followed by a new line. `{}` is replaced by the first
    // argument after the format string.
    println!("Hello {}", lang);

    // Two placeholders need two arguments, filled in from left to right.
    println!("Hello {} {}", lang, lang);

    // Shortcut: put the variable's name inside the braces instead of after the string.
    // This only works for plain variable names, not for expressions like `x * x`.
    println!("Hello {lang}");

    // ---------------------------------------------------------------------------
    // Positional arguments
    // ---------------------------------------------------------------------------

    let x = 2; // the number to square

    // A number inside the braces picks an argument by position, counting from 0:
    // {0} is `x`, the first argument; {1} is `x * x`, the second. {0} is used twice.
    // Prints "2 * 2 = 4".
    println!("{0} * {0} = {1}", x, x * x);

    // ---------------------------------------------------------------------------
    // Printing a struct: {:?} and {:#?}
    // ---------------------------------------------------------------------------

    // Build a Lang. This `lang` shadows the string `lang` above.
    let lang = Lang {
        language: String::from("rust"), // String::from turns a string literal into a String
        version: String::from("1.8.3"), // the version from the video
    };

    // {:?} uses the Debug trait to print the whole struct on one line:
    //     Lang { language: "rust", version: "1.8.3" }
    println!("{:?}", lang);

    // {:#?} is "pretty" Debug: the same thing with line breaks and indentation:
    //     Lang {
    //         language: "rust",
    //         version: "1.8.3",
    //     }
    println!("{:#?}", lang);

    // The variable-name shortcut works with these too.
    println!("{lang:?}");
}

// ---------------------------------------------------------------------------
// cargo fmt
// ---------------------------------------------------------------------------
//
// `cargo fmt` reformats every .rs file in the project to the standard Rust style: it fixes
// indentation and removes unnecessary line breaks. Editors can run it on every save
// (in VS Code with rust-analyzer: the setting "editor.formatOnSave": true).
//
//     cargo fmt            format all the code in the project
//     cargo fmt --check    only report files that need formatting, without changing them
