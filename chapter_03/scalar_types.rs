
// SCALAR TYPES
// Represent a single type
// 1. integers
// Signed: i8, i16, i32 (default), i64, i128, isize
// Unsigned (Only positive): u8, i16, u32, u64, u128, usize
// 2. floating-point numbers
// All are signed: f32, f64 (default)
// 3. Booleans (true, false)
// 4. characters.
// 4 bytes in size which stores unicode scalar values which means it can store a lot more than just ASCII.

fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");

    //  Integer division truncates toward zero to the nearest integer
    // Examples:
    // -5 / 3; // Results in -1
    let truncated = 7/3; // Results in 2
    println!("The value of truncated is: {truncated}");

    let t = true;
    let f: bool = false; // explicit annotation
    println!("The value of f is: {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("Different characters: {c} {z} {heart_eyed_cat}");
}
