
// CONSTANTS VS IMMUTABLES
// 1. mut can't be used with constants
// 2. use const instead of let
// 3. data type must be provided
// 4. can be declared in any scope e.g., global
// 5. can only be set to a constant expression not the result of a value that could only be computed at runtime.


// Naming convention: all uppercase with underscores between words
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}")
}