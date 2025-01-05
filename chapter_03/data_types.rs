

fn main() {
    // In cases when many types are possible, such as when we convert a String to a numeric type using parse, we must add a type annotation, like this:
    let guess: u32 = "42".parse().expect("Not a number!");
    // if u32 is not mentioned rust will throw an error
    println!("{guess}");
}