// import standard input output library
// :: similar to object dot notation  ex: std.io , String.new()
use std::io;

// entry function main
fn main() {
    println!("Guess the number!");
    
    println!("What is your guess:");

    /* create variable 
    - immutable by default 
    - mutable by adding 'mut'
    */
    let mut guess = String::new();

    // Rust naming convention for constants: 
    // const SECONDS_THREE_HOURS: u32 = 360 * 3;

    // if std::io was not imported the next line would be written as std::io::stdin instead
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // dot notation here - need to come back to explain how it is different from ::
    // create gh Issue #1
    
    println!("Your guess: {}", guess);
}
