
// In function signatures, we must declare the type of each parameter.
fn print_labeled_measurement (value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn funky_func() -> i32 {
    return 10;
    5 // no semicolon
}

fn main() {
    my_function();
    print_labeled_measurement(5, 'h');
    let funky_value = funky_func();
    println!("{funky_value}")
}

fn my_function() {
    println!("My first function..")
}
