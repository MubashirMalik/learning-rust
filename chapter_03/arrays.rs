

// COMPOUND TYPES - ARRAYS
// 1. Must have same type
// 2. They too have fixed length
// 3. Store data on stack


fn main () {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let fill = [3, 4];

    let first = a[0];
    let second = fill[1];
    println!("{first}, {second}");
}