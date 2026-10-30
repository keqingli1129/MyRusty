// Common collections: vectors, strings, and hash maps.

use std::collections::HashMap; // HashMap isn't in the prelude, so it has to be imported

pub fn run() {
    // ---------------------------------------------------------------------------
    // Vec<T>: a growable array on the heap
    // ---------------------------------------------------------------------------

    let mut v: Vec<i32> = Vec::new(); // an empty vector; the annotation says what it will hold
    v.push(1); // push appends to the end
    v.push(2);
    v.push(3);
    let mut w = vec![10, 20, 30]; // the vec! macro builds a vector with these elements

    let last = v.pop(); // pop removes the last element and returns it as Some(3)
    let removed = w.remove(0); // remove(index) takes out that element and shifts the rest left
    println!("v = {v:?}, last = {last:?}, w = {w:?}, removed = {removed}");

    // Square brackets index directly and panic if the index is out of range.
    println!("w[1] = {}", w[1]);
    // get(index) returns an Option instead, so a bad index is a None, not a crash.
    println!("w.get(1) = {:?}, w.get(99) = {:?}", w.get(1), w.get(99));

    // ---------------------------------------------------------------------------
    // String and &str
    // ---------------------------------------------------------------------------

    // String owns a growable UTF-8 buffer on the heap. &str is a borrowed view of string data.
    let mut s = String::from("Hello");
    s.push_str(", world"); // push_str appends a &str
    s.push('!'); // push appends a single char
    let slice: &str = &s[0..5]; // borrow part of the String as a &str
    println!("s = {s}, slice = {slice}");

    // format! works like println! but returns the text as a String instead of printing it.
    let name = "Bo";
    let greeting = format!("{s} I'm {name}.");
    println!("{greeting}");

    // ---------------------------------------------------------------------------
    // HashMap<K, V>: key-value pairs
    // ---------------------------------------------------------------------------

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 10); // insert adds a key and its value
    scores.insert(String::from("red"), 50);

    // get returns Option<&V>: Some(&value) if the key exists, None otherwise.
    match scores.get("blue") {
        Some(score) => println!("blue = {score}"),
        None => println!("no blue"),
    }

    // The entry API: or_insert puts in a value only if the key is missing,
    // then returns a mutable reference to the value either way.
    let text = "the cat and the dog and the bird";
    let mut counts: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0); // 0 the first time we see this word
        *count += 1; // `*` dereferences: add 1 to the value inside the map
    }
    println!(
        "the = {}, and = {}, cat = {}",
        counts["the"], counts["and"], counts["cat"]
    );
}
