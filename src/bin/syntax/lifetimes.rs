// Lifetimes.

// Without annotations this doesn't compile: the compiler can't tell whether the returned
// reference comes from x or y, so it can't check how long it stays valid.
//     fn longest(x: &str, y: &str) -> &str
//     error[E0106]: missing lifetime specifier
//
// `'a` is a lifetime parameter, declared in < > like a generic type. It says: the result
// lives at least as long as the shorter-lived of x and y.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

// Lifetime elision: with exactly one reference parameter, the compiler assumes the output
// borrows from it, so no annotation is needed. Written out, this is
//     fn first_char<'a>(s: &'a str) -> &'a str
fn first_char(s: &str) -> &str {
    &s[0..1]
}

// A struct that holds a reference needs a lifetime: an Excerpt can't outlive the text it points to.
struct Excerpt<'a> {
    part: &'a str,
}

pub fn run() {
    let a = String::from("a long string");
    let result;
    {
        let b = String::from("short");
        result = longest(&a, &b); // fine: result is used while both a and b are alive
        println!("longest = {result}");
    }
    // Using result here would fail, because b has been dropped:
    //     error[E0597]: `b` does not live long enough

    println!("first_char = {}", first_char("rust"));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let sentence = novel.split('.').next().unwrap(); // the text before the first '.'
    let excerpt = Excerpt { part: sentence }; // borrows from novel
    println!("excerpt = {}", excerpt.part);

    // 'static means the data lives for the entire program. String literals are &'static str,
    // because their text is stored in the program's binary.
    let forever: &'static str = "I live forever";
    println!("{forever}");
}
