use std::io;

// Generate the nth Fibonacci number.
// 0 1 1 2 3 5 8 13 21 ...
fn main() {
    println!("Which term of the Fibonacci series do you want to generate?");
    let mut nth_term = String::new();

    io::stdin()
        .read_line(&mut nth_term)
        .expect("Failed to read input");

    let nth_term: i32 = nth_term
        .trim() // Remove any whitespace or newline characters
        .parse() // Parse the trimmed string into an integer
        .expect("Please enter a valid integer");

    if nth_term <= 0 {
        println!("Invalid input: {nth_term}");
        return;
    }

    let nth_term_value = get_nth_term_value(nth_term);
    println!("{nth_term} term of fib series is: {nth_term_value}");
}

fn get_nth_term_value (nth_term: i32) -> i32 {
    if nth_term == 1 { // first term
        return 0;
    } else if nth_term == 2 { // second term
        return 1
    }

    let mut first_term = 0;
    let mut second_term = 1; 
    let mut next_term;
    let mut current_term = 2;
    let nth_term_value = loop {
        next_term = first_term + second_term;
        current_term += 1;

        if current_term == nth_term {
            break next_term;
        }
       
        first_term = second_term;
        second_term = next_term;
    };

    
    nth_term_value
}