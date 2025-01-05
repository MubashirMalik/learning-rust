
// COMPOUND TYPES - TUPLES
// General way of grouping together a number of values with a variety of types into one compound type
// Can't grow or shrink in size (like static C++ arrays)
// Empty tuple is called unit tuple

fn main() {
    // annotation is optional here
    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    let x = tup1.0;
    println!("Value at 0th index: {x}");

    let tup2 = ("Mubashir", 26, 182.88);
    let (name, age, height) = tup2; // destructuring 

    println!("My name is {name}. I am {age} years old. My height is {height}cms.");

    let empty_tuple: () = ();
}