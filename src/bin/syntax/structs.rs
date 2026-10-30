// Structs and implementation blocks.

// A struct with named fields. #[derive(Debug)] lets us print it with {:?}.
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
}

// A tuple struct: fields have no names, only positions.
struct Color(u8, u8, u8);

// A unit-like struct: no fields at all. Useful as a marker type.
struct Marker;

// A struct for the methods example.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// `impl` holds the methods and associated functions for a type.
impl Rectangle {
    // An associated function: no `self`. Called with ::, as Rectangle::square(3).
    // Constructors like this are commonly named `new`.
    fn square(size: u32) -> Self {
        // `Self` means "the type this impl is for", here Rectangle.
        Self {
            width: size,
            height: size,
        }
    }

    // `&self` borrows the value to read it. Short for `self: &Self`.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // `&mut self` borrows the value to change it.
    fn grow(&mut self, by: u32) {
        self.width += by;
        self.height += by;
    }

    // `self` takes ownership: the Rectangle is consumed and can't be used afterwards.
    fn into_tuple(self) -> (u32, u32) {
        (self.width, self.height)
    }
}

pub fn run() {
    let name = String::from("Ana");
    let email = String::from("ana@example.com");
    // Field init shorthand: `name` alone means `name: name` when the variable has the field's name.
    let user1 = User {
        name,
        email,
        active: true,
    };
    println!("{user1:?}");

    // Struct update syntax: `..user1` fills in every field not listed from user1.
    let user2 = User {
        email: String::from("bo@example.com"),
        ..user1 // name is moved from user1, active is copied
    };
    println!("{user2:?}");
    println!("{} <{}>, active: {}", user2.name, user2.email, user2.active); // dot reads a field
    // user1.name was moved into user2, so user1 as a whole can't be used anymore.

    // Tuple struct fields are read by position.
    let red = Color(255, 0, 0);
    println!("red = ({}, {}, {})", red.0, red.1, red.2);

    let _marker = Marker; // a unit-like struct is created by just naming it

    // Calling methods.
    let mut rect = Rectangle::square(3); // associated function, with ::
    println!("area = {}", rect.area()); // method, with .
    rect.grow(2); // rect must be `mut` for a &mut self method
    println!("after grow: {rect:?}, area = {}", rect.area());
    let (w, h) = rect.into_tuple(); // rect is moved into the method here
    println!("w = {w}, h = {h}");
    // rect.area(); // error[E0382]: borrow of moved value: `rect`
}
