// The `cartesian` module from docs/Ownership.txt ("Modules and privacy" onwards).
// main.rs says `mod cartesian;`, so the compiler looks for this file, cartesian.rs,
// (or cartesian/mod.rs) in the same folder.
// Everything in a module is private by default; `pub` marks what other modules can use.

// `#[derive(...)]` asks the compiler for automatic implementations of these traits:
//   Debug     - the value can be printed with {:?}
//   Clone     - the value can be copied explicitly with .clone()
//   Copy      - the value is copied implicitly (byte for byte) instead of being moved
//   PartialEq - two values can be compared with ==
// Copy requires Clone, so both have to be listed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    // `pub` on the struct makes the type usable outside this module.
    pub x: i32, // `pub` on a field lets other modules read it, write it, and construct with it
    pub y: i32, // without `pub`: error[E0616]: field `y` of struct `Point` is private
}

// An impl block holds the methods and associated functions for Point.
// It's separate from the struct declaration, but has to be in the same crate.
impl Point {
    // Factory function: an associated function with no `self`, called as Point::origin().
    pub fn origin() -> Point {
        Point { x: 0, y: 0 } // the final expression builds and returns the Point
    }

    // `&self` is a shared reference: a read-only pointer to the Point, so nothing is copied.
    pub fn show(&self) {
        // Dot notation dereferences `self` automatically; there's no `->` like in C.
        println!("({}, {})", self.x, self.y);
    }

    // `&mut self` is a mutable (exclusive) reference, which is needed to change the fields.
    // Other parameters come after `self`.
    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.x += dx; // shift horizontally by dx
        self.y += dy; // shift vertically by dy
    }
}
