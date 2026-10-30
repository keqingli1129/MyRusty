// Smart pointers, Deref coercion, and custom destructors.

use std::cell::RefCell; // interior mutability for one thread
use std::ops::Deref; // the trait behind the `*` operator
use std::rc::Rc; // reference counting for one thread
use std::sync::{Arc, Mutex, RwLock}; // the thread-safe equivalents
use std::thread;

// Our own smart pointer, to show how Deref works.
struct MyBox<T>(T);

// Implementing Deref makes `*my_box` give the inner value, so MyBox acts like a reference.
impl<T> Deref for MyBox<T> {
    type Target = T; // what `*` produces

    fn deref(&self) -> &T {
        &self.0 // a reference to the wrapped value
    }
}

fn hello(name: &str) {
    println!("hello, {name}");
}

// A type with a custom destructor.
struct Noisy(&'static str);

// The Drop trait runs code automatically when a value goes out of scope.
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

pub fn run() {
    // Box<T> puts a value on the heap; the Box itself (a pointer) stays on the stack.
    let boxed = Box::new(5);
    println!("boxed = {}", *boxed + 1); // `*` reads the value it points to

    let my = MyBox(String::from("Rust"));
    println!("*my = {}", *my); // *my calls my.deref() behind the scenes
    // Deref coercion: &MyBox<String> -> &String -> &str, converted automatically to fit hello().
    hello(&my);

    // Rc<T>: several owners of the same value. Rc::clone copies the pointer, not the data,
    // and adds 1 to a counter. The value is freed when the count drops to 0. One thread only.
    let shared = Rc::new(String::from("shared data"));
    let owner2 = Rc::clone(&shared);
    println!("{owner2}, strong count = {}", Rc::strong_count(&shared)); // 2

    // RefCell<T>: change data through a shared reference. The borrow rules are checked
    // at run time instead of compile time; breaking them panics.
    let cell = RefCell::new(vec![1, 2]);
    cell.borrow_mut().push(3); // borrow_mut() hands out a temporary &mut
    println!("cell = {:?}", cell.borrow()); // borrow() hands out a temporary &

    // Rc<RefCell<T>> together: several owners that can all change the value.
    let counter = Rc::new(RefCell::new(0));
    let c2 = Rc::clone(&counter);
    *c2.borrow_mut() += 5; // changes the value that counter also sees
    println!("counter = {}", counter.borrow());

    // Arc<T> is the atomic, thread-safe Rc. Mutex<T> lets only one thread at a time use the data.
    let total = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let total = Arc::clone(&total); // each thread gets its own Arc pointing at the same Mutex
        handles.push(thread::spawn(move || {
            *total.lock().unwrap() += 1; // lock() waits for the mutex; unlocks at end of statement
        }));
    }
    for h in handles {
        h.join().unwrap(); // wait for each thread to finish
    }
    println!("total = {}", total.lock().unwrap()); // 4

    // RwLock<T>: many readers at once, or one writer.
    let config = RwLock::new(String::from("v1"));
    println!("read: {}", config.read().unwrap()); // read() for shared access
    config.write().unwrap().push_str("-updated"); // write() for exclusive access
    println!("after write: {}", config.read().unwrap());

    // Drop runs in reverse order of creation when the scope ends.
    {
        let _a = Noisy("a");
        let _b = Noisy("b");
        println!("leaving the scope");
    } // prints "dropping b", then "dropping a"
}
