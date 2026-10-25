// Code that follows along with docs/Ownership.txt (Rust Curious, lesson 4: tuples, structs,
// methods, privacy and move-only types).
//
// Run it with:  cargo run --bin ownership
//
// The program is split into modules, like in the video:
//   main.rs       this file, the root module: one function per section, and main()
//   cartesian.rs  the finished Point type ("Modules and privacy" onwards)
//   video.rs      the Video type and the practice exercise
// It lives in its own folder because cargo treats every .rs file directly in src/bin/ as a
// separate program. Files in a subfolder like this one can be modules instead.
//
// Early sections declare Point inside the function, as it looked at that point in the video.
// A struct declared inside a function only exists in that function, and it hides the
// finished cartesian::Point imported below.

mod cartesian; // declare the module; the compiler reads its code from cartesian.rs
mod video; // the same for video.rs

use cartesian::Point; // import the name, so we can write Point instead of cartesian::Point
use video::Video; // import Video the same way

// ---------------------------------------------------------------------------
// Tuples
// ---------------------------------------------------------------------------

// Returns two values at once by bundling them in a tuple.
// `(i32, bool)` is a tuple TYPE: no declaration needed, just types in parens.
fn foo() -> (i32, bool) {
    (0, true) // `(0, true)` is a tuple EXPRESSION that builds a value of that type
}

fn tuples() {
    let x = foo(); // x holds the whole tuple
    // The compiler names the fields by position: .0 is the i32, .1 is the bool.
    println!("x.0 = {}, x.1 = {}", x.0, x.1);
    // A tuple is not a list or array. Its fields can have different types, so you can't
    // index it with a variable (x[i]) or iterate over it.
}

// ---------------------------------------------------------------------------
// Destructuring tuples
// ---------------------------------------------------------------------------

fn destructuring_tuples() {
    // `(a, b)` on the left is a tuple PATTERN: it splits the tuple into two new variables.
    let (a, b) = foo();
    // a got the i32 and b got the bool.
    println!("a = {}, b = {}", a, b);

    // chars() gives an iterator over the characters of a string.
    for c in "Hey".chars() {
        print!("{} ", c); // c is the next character on each iteration
    }
    println!(); // end the line

    // enumerate() is an "iterator adaptor": it wraps each item in a tuple (count, item).
    // The for loop's pattern `(i, c)` destructures each tuple as it arrives.
    for (i, c) in "Hey".chars().enumerate() {
        print!("{}:{} ", i, c); // i is the count (from 0), c is the character
    }
    println!(); // end the line
}

// ---------------------------------------------------------------------------
// Patterns, types and expressions
// ---------------------------------------------------------------------------

fn patterns_types_expressions() {
    // let  PATTERN    : TYPE              = EXPRESSION
    let (a, b, c): (i32, bool, char) = (0, true, 'H');
    // Each part is built from smaller parts of the same kind:
    //   expressions 0, true, 'H'  ->  tuple expression (0, true, 'H')
    //   types i32, bool, char     ->  tuple type (i32, bool, char)
    //   patterns a, b, c          ->  tuple pattern (a, b, c)
    println!("a = {}, b = {}, c = {}", a, b, c);

    // Common mistakes, because the type and the pattern are separate pieces of grammar:
    //   let (a: i32, b: bool) = (0, true);   // error: type ascriptions can't go in a pattern
    //   for i: u32 in 0..10 {}               // error: the grammar is `for PATTERN in EXPRESSION`

    // A tuple with one field needs a trailing comma. Without it, (5) is just 5 in parens.
    let single: (i32,) = (5,);
    // Its only field is .0.
    println!("single.0 = {}", single.0);
}

// ---------------------------------------------------------------------------
// Unit ()
// ---------------------------------------------------------------------------

// These two declarations mean the same thing: with no return type, a function returns unit.
fn returns_nothing() {}
fn returns_unit() -> () {}

fn unit() {
    // Pattern `()`, type `()`, expression `()`. Completely pointless, but legal.
    let (): () = ();
    // Both functions give back the single value of type unit.
    let r1: () = returns_nothing();
    let r2: () = returns_unit();
    // {:?} prints using the Debug trait; unit prints as ().
    println!("returns_nothing() = {:?}, returns_unit() = {:?}", r1, r2);
    // Unit has only one possible value, so there is nothing to store: it's zero-sized.
    println!("size of () is {} bytes", std::mem::size_of::<()>());
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

fn structs() {
    // Point as first declared in the video: no derives, no methods.
    struct Point {
        x: i32, // each field has a name and a type, separated by commas
        y: i32, // the trailing comma is optional, but idiomatic with one field per line
    }

    // A function that returns a Point. The outer `Point` is the return type; the
    // `Point { ... }` in the body is the expression that constructs the value.
    fn origin() -> Point {
        Point { x: 0, y: 0 } // final expression, so it's the return value
    }

    // Every field must be given a value: there's no uninitialised or half-built struct.
    let p = Point { x: 1, y: 2 };
    // Fields are read with dot notation.
    println!("p = ({}, {})", p.x, p.y);

    // Call origin to get another Point. This new `p` shadows the old one.
    let p = origin();
    // Print its fields.
    println!("origin = ({}, {})", p.x, p.y);
}

// ---------------------------------------------------------------------------
// Inherited mutability
// ---------------------------------------------------------------------------

fn inherited_mutability() {
    // From here on, Point is the finished one from cartesian.rs (its fields are `pub`).
    let p = Point::origin(); // an immutable variable
    // p.x = 5; // error[E0594]: cannot assign to `p.x`, as `p` is not declared as mutable
    println!("immutable p = ({}, {})", p.x, p.y); // reading is fine

    // `let mut` makes the variable mutable, and every field inherits that mutability.
    // You can't mark single fields as mutable or immutable; use private fields for that.
    let mut p = Point::origin();
    p.x = 5; // now allowed
    p.y = 7; // also allowed
    println!("mutable p = ({}, {})", p.x, p.y); // print the changed fields
}

// ---------------------------------------------------------------------------
// Destructuring structs
// ---------------------------------------------------------------------------

fn destructuring_structs() {
    let p = Point { x: 3, y: 4 }; // a Point to take apart

    // A struct pattern: field x goes into new variable a, field y into new variable b.
    let Point { x: a, y: b } = p;
    println!("a = {}, b = {}", a, b);

    // Shortcut when the variable has the field's name: `x` means `x: x`.
    // These two lines are equivalent:
    //     let Point { x: x, y: y } = p;
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);

    // `_` matches anything and ignores it: this only makes the variable `x2`.
    let Point { x: x2, y: _ } = p;
    println!("x2 = {}", x2);

    // Every field must be mentioned. This is an error, because y isn't:
    //     let Point { x } = p; // error[E0027]: pattern does not mention field `y`
    // That's useful: adding a field later makes the compiler point at every such pattern.
    // `..` means "ignore all the other fields":
    let Point { x: x3, .. } = p;
    println!("x3 = {}", x3);
}

// ---------------------------------------------------------------------------
// Tuple-structs, newtypes
// ---------------------------------------------------------------------------

// Newtypes: tuple-structs wrapping a single f64, so the two units can't be mixed up.
struct Kilograms(f64);
struct Pounds(f64);

// Only accepts Kilograms. Passing Pounds is a compile error:
//     check_luggage(Pounds(50.0)); // error[E0308]: mismatched types: expected `Kilograms`, found `Pounds`
fn check_luggage(weight: Kilograms) -> bool {
    weight.0 <= 23.0 // .0 is the f64 inside; true if the bag is within the limit
}

fn tuple_structs() {
    // A tuple-struct: the type has a name, the fields don't. Written with parens.
    struct Point(i32, i32);
    let p = Point(1, 2); // construct it like a function call
    // Fields are accessed by position, like a tuple, but Point is a distinct named type.
    println!("p.0 = {}, p.1 = {}", p.0, p.1);

    // Converting pounds to kilograms has to be done on purpose.
    let bag = Pounds(50.0); // a weight in pounds
    let kg = Kilograms(bag.0 * 0.4536); // unwrap, convert, and wrap in the other type
    println!("{} lb = {:.1} kg", bag.0, kg.0); // {:.1} means one decimal place
    // Check the converted weight against the limit.
    println!("within the 23 kg limit? {}", check_luggage(kg));
}

// ---------------------------------------------------------------------------
// Unit-like structs
// ---------------------------------------------------------------------------

fn unit_like_structs() {
    // A struct with no fields at all. Like unit, it's zero-sized with only one value.
    struct Foo;
    let foo = Foo; // the expression `Foo` constructs the one value
    let Foo = foo; // the pattern `Foo` "destructures" it (there's nothing inside)
    // Useful later with traits, when an API wants a value but there's no data to give.
    println!("size of Foo is {} bytes", std::mem::size_of::<Foo>());
}

// ---------------------------------------------------------------------------
// Move-only by default
// ---------------------------------------------------------------------------

fn move_only_by_default() {
    // A Point with no derives: like every struct, it's move-only by default.
    struct Point {
        x: i32, // x coordinate
        y: i32, // y coordinate
    }

    // Takes a Point BY VALUE: the caller's Point is moved into p.
    fn show(p: Point) {
        println!("({}, {})", p.x, p.y);
    }

    let p = Point { x: 1, y: 2 }; // make a Point
    show(p); // the value of p moves into show(); p is no longer available
    // show(p); // error[E0382]: use of moved value: `p`
    //          // move occurs because `p` has type `Point`, which does not implement the `Copy` trait
}

// ---------------------------------------------------------------------------
// Deriving Copy and Clone
// ---------------------------------------------------------------------------

fn deriving_copy_clone() {
    // `#[...]` is an attribute on the item that follows. `derive` asks the compiler for
    // automatic trait implementations. Copy needs Clone, so we derive both.
    #[derive(Copy, Clone)]
    struct Point {
        x: i32, // plain data, so copying the bytes is fine
        y: i32, // the same
    }

    // The same by-value function as before.
    fn show(p: Point) {
        println!("({}, {})", p.x, p.y);
    }

    let p = Point { x: 1, y: 2 }; // make a Point
    show(p); // passes a copy; p is still valid
    show(p); // so this works now
    let q = p.clone(); // Clone: an explicit copy through a method call
    show(q); // q is an independent copy

    // Which types are Copy?
    let t = (1, 'a'); // tuples are Copy if all their fields are
    let t2 = t; // copies, so t is still usable
    let arr = [1, 2, 3]; // arrays are Copy if their element type is
    let arr2 = arr; // copies, so arr is still usable
    println!("t.0 = {}, t2.0 = {}, arr[0] = {}, arr2[0] = {}", t.0, t2.0, arr[0], arr2[0]);
    // All scalar types are Copy. Structs and enums are not, unless they opt in with derive.
    // A realistic declaration usually derives several traits; see cartesian.rs.
}

// ---------------------------------------------------------------------------
// Methods
// ---------------------------------------------------------------------------

fn methods() {
    // Point with a method taking `self` by value (each call gets a copy).
    #[derive(Copy, Clone)]
    struct Point {
        x: i32, // x coordinate
        y: i32, // y coordinate
    }

    // Methods go in an impl block, not inside the struct declaration.
    impl Point {
        // `self` is a value of the type after `impl`, here Point.
        fn show(self) {
            println!("({}, {})", self.x, self.y);
        }
    }

    let p = Point { x: 1, y: 2 }; // make a Point
    p.show(); // dot notation: whatever is left of the dot becomes `self`
    Point::show(p); // the same call as a plain function in Point's namespace
}

// ---------------------------------------------------------------------------
// Shared references, &self
// ---------------------------------------------------------------------------

fn shared_references() {
    // cartesian::Point's show() takes `&self`: a read-only pointer, no copy of the struct.
    let p = Point { x: 1, y: 2 };
    p.show(); // dot notation takes the reference to p automatically
    Point::show(&p); // as a plain function, we have to pass `&p` ourselves
    // Point::show(p); // error[E0308]: mismatched types: expected `&Point`, found `Point`
}

// ---------------------------------------------------------------------------
// Mutable references, &mut self
// ---------------------------------------------------------------------------

fn mutable_references() {
    // offset() takes `&mut self`: exclusive access, which is required to change fields.
    // let p = Point::origin();
    // p.offset(3, 4); // error[E0596]: cannot borrow `p` as mutable, as it is not declared as mutable
    let mut p = Point::origin(); // `mut`, so a mutable reference can be taken
    p.offset(3, 4); // dot notation takes `&mut p` automatically
    p.show(); // the change is visible: prints (3, 4)
}

// ---------------------------------------------------------------------------
// Factory functions
// ---------------------------------------------------------------------------

fn factory_functions() {
    // Functions in an impl block are "associated functions". They don't need `self`.
    // origin() is one: it builds a Point and lives in Point's namespace.
    let p = Point::origin();
    p.show(); // prints (0, 0)
    // So associated functions can take: no self, self, &self or &mut self.
    // That choice controls how a type can be used.
}

// ---------------------------------------------------------------------------
// Modules and privacy, public items, private fields
// ---------------------------------------------------------------------------
//
// Privacy is enforced at module boundaries, so Point moved into its own module, `cartesian`.
// There are two ways to write a module:
//     mod cartesian { ... }   // the body inline, in curly braces (see `mod ride` below)
//     mod cartesian;          // the body in cartesian.rs or cartesian/mod.rs (used above)
// Modules can be nested, so the folders end up matching the module tree.
//
// Everything is private by default. Before `pub` was added in cartesian.rs:
//     use cartesian::Point; // error[E0603]: struct `Point` is private
// Private items are visible only to their own module and its sub-modules.
//
// A `pub struct` still has private fields unless each field is marked `pub`:
//     p.x                   // error[E0616]: field `x` of struct `Point` is private
//     Point { x: 0, y: 0 }  // error[E0451]: fields `x` and `y` of struct `Point` are private
// For plain data like Point, public fields make sense, so cartesian.rs uses `pub x` and `pub y`.

fn modules_and_privacy() {
    // The fields are `pub`, so main (another module) can construct a Point directly
    let mut p = Point { x: 10, y: 20 };
    p.x += 1; // and read and write its fields
    p.show(); // prints (11, 20)
}

// ---------------------------------------------------------------------------
// Resource types
// ---------------------------------------------------------------------------

// The ride API, written as an inline module this time.
mod ride {
    // Compare two ways to declare Token. Both are zero-sized:
    //     pub struct Token;      // unit-like: anyone can make one just by writing `Token`
    //     pub struct Token(());  // one PRIVATE field of type unit: only this module can make one
    pub struct Token(());

    // The only public way to get a Token.
    pub fn purchase() -> Token {
        println!("Enjoy your ride!"); // pretend we took payment
        Token(()) // allowed here, because we're inside the `ride` module
    }

    impl Token {
        // Takes `self` BY VALUE, so calling redeem consumes the token.
        pub fn redeem(self) {
            println!("Weeee!!!"); // enjoy the ride
        } // the token is gone after this
    }
}

fn resource_types() {
    let token = ride::purchase(); // buy a ride
    token.redeem(); // use it; `token` has now moved into redeem
    // token.redeem(); // error[E0382]: use of moved value: `token`
    //                 // note: `Token::redeem` takes ownership of the receiver `self`, which moves `token`

    let token1 = ride::purchase(); // buy another
    let token2 = token1; // assignment is a MOVE, not a copy: token1 is now invalid
    // token1.redeem(); // error[E0382]: use of moved value: `token1`
    token2.redeem(); // only token2 can be redeemed

    // Trying to make our own Token for a free ride:
    //     let free = ride::Token(());
    // The video's compiler said "cannot initialize a tuple struct which contains private fields";
    // the current one says error[E0603]: tuple struct constructor `Token` is private.

    // No data inside at all, but it still acts as a resource: having a Token means you
    // bought a ride and haven't used it yet.
    println!("size of Token is {} bytes", std::mem::size_of::<ride::Token>());
}

// ---------------------------------------------------------------------------
// Destructors, Drop trait
// ---------------------------------------------------------------------------
//
// Cleaning up a resource when its owner goes out of scope is done by a destructor, written
// by implementing the Drop trait. The video leaves the details (drop flags, panics while
// unwinding, leaks) for lesson 5, Destruction.

// ---------------------------------------------------------------------------
// References to private fields; Clone trait; String vs &str
// ---------------------------------------------------------------------------

fn references_to_private_fields() {
    // String owns a heap allocation, so it's move-only: copying the struct would leave two
    // Strings pointing at the same bytes, and one could free them under the other.
    let s = String::from("hello"); // allocate a String on the heap
    let moved = s; // MOVE: `s` is no longer usable
    // println!("{}", s); // error[E0382]: borrow of moved value: `s`
    let cloned = moved.clone(); // CLONE: a deep copy into a new heap allocation
    println!("moved = {}, cloned = {}", moved, cloned); // both are valid, independent Strings
    // Copy is implicit and cheap (copy the bytes). Clone is explicit and can do real work.
    // A move transfers ownership and invalidates the old variable.

    let r: &String = &moved; // a shared reference to the String struct
    let slice: &str = r; // coerces automatically to a string slice of its UTF-8 bytes
    println!("as &str: {}", slice); // print through the slice

    // Video keeps its title private and hands out a &str, so nothing is copied.
    let v = Video::new(String::from("Rust Curious 4: Structs")); // the String moves into the Video
    let title: &str = v.title(); // borrow the title; v is read-only while `title` is in use
    println!("video title: {}", title); // print the borrowed title
}

// ---------------------------------------------------------------------------
// Practice exercise: likes and dislikes (implemented in video.rs)
// ---------------------------------------------------------------------------

fn practice_exercise() {
    let mut v = Video::new(String::from("Enums")); // `mut`, because the add_ methods take &mut self
    v.add_like(); // 1 like
    v.add_like(); // 2 likes
    v.add_like(); // 3 likes
    v.add_dislike(); // 1 dislike
    let (likes, dislikes) = v.likes_dislikes(); // destructure the returned tuple
    // Print the counts and the ratio.
    println!(
        "{}: {} likes, {} dislikes, ratio {}",
        v.title(),
        likes,
        dislikes,
        v.like_ratio()
    );
}

// ---------------------------------------------------------------------------
// main: run every section in transcript order
// ---------------------------------------------------------------------------

fn main() {
    println!("--- Tuples ---"); // section header
    tuples(); // run the section

    println!("\n--- Destructuring tuples ---");
    destructuring_tuples();

    println!("\n--- Patterns, types and expressions ---");
    patterns_types_expressions();

    println!("\n--- Unit () ---");
    unit();

    println!("\n--- Structs ---");
    structs();

    println!("\n--- Inherited mutability ---");
    inherited_mutability();

    println!("\n--- Destructuring structs ---");
    destructuring_structs();

    println!("\n--- Tuple-structs, newtypes ---");
    tuple_structs();

    println!("\n--- Unit-like structs ---");
    unit_like_structs();

    println!("\n--- Move-only by default ---");
    move_only_by_default();

    println!("\n--- Deriving Copy and Clone ---");
    deriving_copy_clone();

    println!("\n--- Methods ---");
    methods();

    println!("\n--- Shared references, &self ---");
    shared_references();

    println!("\n--- Mutable references, &mut self ---");
    mutable_references();

    println!("\n--- Factory functions ---");
    factory_functions();

    println!("\n--- Modules and privacy ---");
    modules_and_privacy();

    println!("\n--- Resource types ---");
    resource_types();

    println!("\n--- References to private fields ---");
    references_to_private_fields();

    println!("\n--- Practice exercise ---");
    practice_exercise();
}
