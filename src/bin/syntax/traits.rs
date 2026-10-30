// Generics and traits.

use std::fmt::Display; // the trait for types that can be printed with {}

// ---------------------------------------------------------------------------
// Generics
// ---------------------------------------------------------------------------

// A generic function: `<T>` declares a type parameter, filled in by the caller.
// `T: PartialOrd + Copy` is a trait bound: T must support < and > and be copyable.
fn largest<T: PartialOrd + Copy>(items: &[T]) -> T {
    let mut max = items[0]; // Copy lets us copy the element out of the slice
    for &item in items {
        if item > max {
            max = item;
        }
    }
    max
}

// A generic struct: the same code works for Pair<i32>, Pair<f64>, and so on.
#[derive(Debug)]
struct Pair<T> {
    a: T,
    b: T,
}

// A generic enum. The standard Option<T> and Result<T, E> are defined just like this.
#[derive(Debug)]
#[allow(dead_code)] // we only ever build Full, so this silences "variant never constructed"
enum Slot<T> {
    Empty,
    Full(T),
}

// ---------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------

// A trait declares shared behavior, like an interface in other languages.
trait Speak {
    fn name(&self) -> String; // a required method: only the signature, ending in ;

    // A default method: implementors get it for free and may override it.
    fn speak(&self) -> String {
        format!("{} makes a sound", self.name())
    }
}

struct Dog;
struct Cat;

// `impl Trait for Type` implements a trait for a type.
impl Speak for Dog {
    fn name(&self) -> String {
        String::from("Dog")
    }
    fn speak(&self) -> String {
        String::from("Woof!") // overrides the default
    }
}

impl Speak for Cat {
    fn name(&self) -> String {
        String::from("Cat")
    } // no speak here, so Cat uses the default
}

// From and Into: the standard traits for converting one type into another.
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}
// Implementing From gives you Into for free, in the other direction.

// ---------------------------------------------------------------------------
// Trait bounds: three ways to write them
// ---------------------------------------------------------------------------

// 1. `impl Trait` in the parameter: "any type that implements Speak".
fn introduce(animal: &impl Speak) {
    println!("introducing: {}", animal.speak());
}

// 2. `+` for several bounds on one type parameter.
fn show_twice<T: Display + Clone>(item: T) {
    let copy = item.clone(); // allowed because of Clone
    println!("{item} {copy}"); // allowed because of Display
}

// 3. A `where` clause moves the bounds after the signature, which reads better when long.
fn compare<T, U>(t: T, u: U) -> String
where
    T: Display,
    U: Display + PartialEq<i32>, // U can be compared with an i32
{
    format!("{t} vs {u}, u == 0? {}", u == 0)
}

// ---------------------------------------------------------------------------
// derive
// ---------------------------------------------------------------------------

// derive generates standard trait implementations for you:
//   Debug - {:?} printing        Clone, Copy - duplicating values
//   PartialEq, Eq - == and !=    Default - a zero/empty value from Point::default()
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run() {
    println!("largest of [3, 9, 2] = {}", largest(&[3, 9, 2])); // T is i32
    println!("largest of ['a', 'z', 'q'] = {}", largest(&['a', 'z', 'q'])); // T is char
    let pair = Pair { a: 1.5, b: 2.5 }; // T is f64
    let slot: Slot<&str> = Slot::Full("gold");
    println!("{pair:?} {slot:?} {:?}", pair.a + pair.b);

    let dog = Dog;
    let cat = Cat;
    println!("{}", dog.speak()); // the overridden method
    println!("{}", cat.speak()); // the default method
    introduce(&dog);

    let boiling = Fahrenheit::from(Celsius(100.0)); // using From directly
    let body: Fahrenheit = Celsius(37.0).into(); // using Into; the annotation picks the target
    println!("100C = {}F, 37C = {:.1}F", boiling.0, body.0); // {:.1}: one decimal place

    show_twice("hi");
    println!("{}", compare("left", 0));

    let p = Point::default(); // Point { x: 0, y: 0 }
    let q = p; // Copy, so p stays usable
    println!("{p:?} == {q:?}? {}", p == q);

    // Trait objects: `dyn Speak` is "some type that implements Speak", decided at run time.
    // Box puts each value on the heap, so different types can share one Vec.
    let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in &animals {
        println!("dyn: {}", animal.speak()); // dynamic dispatch: looks up the right speak()
    }
}
