
// Why? Shadowing

// 1. We can keep same variable name and apply some transformations on original value
// 2. It is also possible to change value type as it actually creates a new variable when we use let again


fn main () {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}