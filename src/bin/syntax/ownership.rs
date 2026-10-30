// Ownership foundations, the Copy trait, borrowing, references, and slices.

// Takes a reference, so it can read the String without taking ownership of it.
fn length(s: &String) -> usize {
    s.len() // the caller still owns s after this returns
}

// Takes a mutable reference, so it can change the caller's String.
fn add_world(s: &mut String) {
    s.push_str(", world"); // push_str appends a &str to the end
}

// Returns a slice: a view into the first word of the string, without copying it.
fn first_word(s: &str) -> &str {
    // split_whitespace() yields the words; next() takes the first one, if there is one.
    s.split_whitespace().next().unwrap_or("") // unwrap_or gives "" for an empty string
}

pub fn run() {
    // ---------------------------------------------------------------------------
    // The three ownership rules
    // ---------------------------------------------------------------------------
    // 1. Each value has an owner (a variable).
    // 2. There is only one owner at a time.
    // 3. When the owner goes out of scope, the value is dropped (its memory is freed).

    {
        let inner = String::from("temporary"); // inner owns this String
        println!("inner = {inner}");
    } // inner goes out of scope here, and the String's heap memory is freed

    // Assigning a heap value MOVES it: s2 becomes the owner and s1 can't be used anymore.
    let s1 = String::from("hello");
    let s2 = s1; // move: only the pointer, length and capacity are copied, not the text
    // println!("{s1}"); // error[E0382]: borrow of moved value: `s1`
    println!("s2 = {s2}");

    // .clone() makes a deep copy, so both variables own their own String.
    let s3 = s2.clone();
    println!("s2 = {s2}, s3 = {s3}");

    // ---------------------------------------------------------------------------
    // The Copy trait
    // ---------------------------------------------------------------------------

    // Types with a known size that live only on the stack (integers, floats, bool, char,
    // and tuples/arrays of those) implement Copy. Assigning them copies instead of moving.
    let a = 5;
    let b = a; // a copy, so a is still usable
    println!("a = {a}, b = {b}");

    // ---------------------------------------------------------------------------
    // Borrowing with references
    // ---------------------------------------------------------------------------

    let mut text = String::from("hello");
    // `&text` creates an immutable reference: borrowing the value to read it.
    println!("length = {}", length(&text));
    // `&mut text` creates a mutable reference: borrowing it to change it. text must be `mut`.
    add_world(&mut text);
    println!("text = {text}");

    // The aliasing rule: at any one time you can have EITHER any number of `&` references
    // OR exactly one `&mut` reference, never both.
    let r1 = &text; // fine
    let r2 = &text; // fine: many readers are allowed
    println!("r1 = {r1}, r2 = {r2}");
    // r1 and r2 aren't used after this point, so their borrow has ended.
    let r3 = &mut text; // fine now, because no immutable reference is still in use
    r3.push('!'); // push adds one char
    println!("r3 = {r3}");
    // If r1 were used after r3 was created:
    //     error[E0502]: cannot borrow `text` as mutable because it is also borrowed as immutable

    // ---------------------------------------------------------------------------
    // Slices
    // ---------------------------------------------------------------------------

    // A string slice, &str, refers to part of a String.
    let sentence = String::from("hello brave world");
    let hello: &str = &sentence[0..5]; // bytes 0 to 4; the range excludes 5
    let world = &sentence[12..]; // from byte 12 to the end
    println!(
        "hello = {hello}, world = {world}, first word = {}",
        first_word(&sentence)
    );

    // An array slice, &[T], refers to part of an array or a vector.
    let numbers = [10, 20, 30, 40, 50];
    let middle: &[i32] = &numbers[1..4]; // elements 1, 2 and 3
    println!("middle = {middle:?}");
}
