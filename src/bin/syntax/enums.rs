// Enums and advanced pattern matching.

// An enum lists every possible variant of a type. Each variant can carry different data.
#[derive(Debug)]
enum Message {
    Quit,                    // no data
    Move { x: i32, y: i32 }, // named fields, like a struct
    Write(String),           // one value, like a tuple struct
    ChangeColor(u8, u8, u8), // several values, like a tuple
}

// `match` is exhaustive: every variant must be handled, or the code doesn't compile.
fn describe(msg: &Message) -> String {
    match msg {
        Message::Quit => String::from("quit"), // pattern => expression
        Message::Move { x, y } => format!("move to ({x}, {y})"), // the pattern pulls out the fields
        Message::Write(text) => format!("write {text:?}"),
        Message::ChangeColor(r, g, b) => format!("color #{r:02x}{g:02x}{b:02x}"), // 02x: 2-digit hex
    }
    // Leaving out a variant: error[E0004]: non-exhaustive patterns
}

// Match patterns for numbers.
fn classify(n: u32) -> String {
    match n {
        0 => String::from("zero"),
        1 | 2 => String::from("one or two"), // `|` matches either pattern
        3..=9 => String::from("a single digit"), // `..=` matches an inclusive range
        big @ 100.. => format!("{big} is big"), // `name @ pattern` binds the matched value to a name
        _ => String::from("something else"),    // `_` is the wildcard: it matches anything left
    }
}

pub fn run() {
    // Build one value of each variant.
    let messages = [
        Message::Quit,
        Message::Move { x: 1, y: 2 },
        Message::Write(String::from("hi")),
        Message::ChangeColor(255, 128, 0),
    ];
    for msg in &messages {
        println!("{msg:?} -> {}", describe(msg));
    }

    for n in [0, 2, 7, 150, 42] {
        println!("{n}: {}", classify(n));
    }

    // `if let` handles one pattern and ignores the rest, without an exhaustive match.
    let msg = Message::Write(String::from("hello"));
    if let Message::Write(text) = &msg {
        println!("if let found text: {text}");
    } else {
        println!("not a Write"); // the else branch is optional
    }

    // `while let` keeps looping as long as the pattern matches.
    let mut stack = vec![1, 2, 3];
    // pop() returns Some(value) until the vector is empty, then None, which ends the loop.
    while let Some(top) = stack.pop() {
        print!("{top} ");
    }
    println!();
}
