// Closures and iterators.

pub fn run() {
    // ---------------------------------------------------------------------------
    // Closures
    // ---------------------------------------------------------------------------

    // A closure is an anonymous function. Parameters go between vertical pipes.
    let add_one = |x: i32| x + 1; // the body can be a single expression
    let square = |x| {
        // types are usually inferred
        x * x // a block body, for more than one line
    };
    println!("add_one(5) = {}, square(4) = {}", add_one(5), square(4));

    // Closures capture variables from the scope around them. This one borrows `offset`.
    let offset = 10;
    let shift = |x: i32| x + offset;
    println!("shift(1) = {}", shift(1));

    // A closure that changes a captured variable must itself be `mut`.
    let mut count = 0;
    let mut increment = || count += 1;
    increment();
    increment();
    println!("count = {count}"); // 2

    // `move` makes the closure take ownership of what it captures.
    // This is needed when the closure outlives the scope, e.g. when sent to another thread.
    let name = String::from("Rust");
    let say_hi = move || println!("hi from {name}"); // name is moved into the closure
    say_hi();
    // println!("{name}"); // error[E0382]: borrow of moved value: `name`

    // ---------------------------------------------------------------------------
    // Iterators
    // ---------------------------------------------------------------------------

    let mut nums = vec![1, 2, 3, 4, 5];

    // iter() yields &T: immutable references; nums is unchanged and still usable.
    let total: i32 = nums.iter().sum();
    println!("total = {total}");

    // iter_mut() yields &mut T: mutable references, to change elements in place.
    for n in nums.iter_mut() {
        *n *= 10; // dereference to change the element itself
    }
    println!("nums = {nums:?}");

    // Iterators are lazy: adapters like filter and map do nothing until something consumes them.
    // collect() consumes the iterator and builds a collection.
    let doubled: Vec<i32> = nums
        .iter() // &i32
        .filter(|&&n| n > 25) // filter gets a &&i32; the pattern `&&n` copies out the i32
        .map(|n| n * 2) // map transforms each item
        .collect(); // the annotation on doubled says to build a Vec<i32>
    println!("doubled = {doubled:?}"); // [60, 80, 100]

    // into_iter() yields T: owned values. It consumes the vector, so nums can't be used after.
    let strings: Vec<String> = nums.into_iter().map(|n| format!("#{n}")).collect();
    println!("strings = {strings:?}");
}
