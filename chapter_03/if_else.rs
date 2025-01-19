

fn main () {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    if number != 0 { // condition must be a bool. if number will throw an error.
        println!("x is greater than zero.");
    } else {
        println!("x is equal to zero.")
    }
}