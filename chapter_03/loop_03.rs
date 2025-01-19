
fn main () {

    'outer_loop: loop {
        println!("outer loop");
        loop {
            println!("middle loop");
            'inner_most_loop: loop {
                println!("inner most loop");
                break 'outer_loop;
                break; // this would have only exited the current loop
            }
        }
    }
}