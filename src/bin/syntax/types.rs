// Scalar and compound data types.

pub fn run() {
    // ---------------------------------------------------------------------------
    // Scalar types: a single value
    // ---------------------------------------------------------------------------

    // Signed integers: i8, i16, i32, i64, i128. The number is the size in bits.
    let a: i8 = -128; // the smallest i8
    let b: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // the biggest i128
    // Unsigned integers can't be negative: u8, u16, u32, u64, u128.
    let c: u8 = 255; // the biggest u8
    // isize and usize are pointer-sized: 64 bits on a 64-bit machine. Indexes and lengths are usize.
    let d: usize = 3;
    println!("i8 {a}, i128 {b}, u8 {c}, usize {d}");

    // Floating-point numbers: f32 and f64. f64 is the default for a literal like 1.5.
    let e: f32 = 1.5;
    let pi = 3.14159; // inferred as f64
    println!("f32 {e}, f64 {pi}");

    // bool is `true` or `false`.
    let is_ok: bool = true;
    // char is one Unicode scalar value, 4 bytes, written in single quotes.
    let letter: char = 'R';
    let crab = '🦀'; // any Unicode character fits, not just ASCII
    println!(
        "bool {is_ok}, chars {letter} {crab}, size of char = {} bytes",
        size_of::<char>()
    );

    // ---------------------------------------------------------------------------
    // Compound types: several values grouped together
    // ---------------------------------------------------------------------------

    // A tuple has a fixed length and each element can have a different type.
    let tup: (i32, f64, char) = (500, 6.4, 'x');
    let first = tup.0; // elements are read by position: .0, .1, .2
    let (_, second, third) = tup; // or destructured with a pattern; `_` skips an element
    println!("tuple: {first} {second} {third}");

    // An array has a fixed length and every element has the same type.
    // The type is written [type; length].
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    let zeros = [0; 3]; // [value; count] repeats a value: [0, 0, 0]
    println!(
        "nums[0] = {}, len = {}, zeros = {:?}",
        nums[0],
        nums.len(),
        zeros
    );
    // Indexing past the end panics at run time: nums[10] would stop the program.
}
