
// Why? Shadowing

// 1. We can keep same variable name and apply some transformations on original value
// 2. It is also possible to change value type as it actually creates a new variable when we use let again


fn main() {
    let x = 5; // 5
    let x = x + 1; // 6

    {
        let x = x * 2; // 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // 6
}