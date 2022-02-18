// import standard input output library
// :: similar to object dot notation  ex: std.io , String.new()
// :: but probably similar to scope resolution
use std::io;
use std::cmp::Ordering;
use rand::Rng;
// $ cargo doc --open 
// to open docs for crates

// entry function main
fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("What is your guess:");

    /* create variable 
    - immutable by default 
    - mutable by adding 'mut'
    */
    let mut guess = String::new();

    // Rust naming convention for constants: 
    // const SECONDS_THREE_HOURS: u32 = 360 * 3;

    // if std::io was not imported the next line would be written as std::io::stdin instead
    /*
    The & indicates that this argument is a reference, 
    which gives you a way to let multiple parts of your code access one piece 
    of data without needing to copy that data into memory multiple times.
    references are immutable by default as well
    hence the mut
    */
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // dot notation here - need to come back to explain how it is different from ::
    // create gh Issue #1
    
    //Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
    //This feature is often used when you want to convert a value from one type to another type.
    //The colon (:) after guess tells Rust we’ll annotate the variable’s type.
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // {} is a placeholder
    // multi {} example - println!("x = {} and y = {}", x, y);
        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// $ cargo update - ignore lock file to update crates minor version