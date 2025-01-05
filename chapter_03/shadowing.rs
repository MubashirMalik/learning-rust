
fn main() {
    let x = 5; // 5
    let x = x + 1; // 6

    {
        let x = x * 2; // 12
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // 6
}