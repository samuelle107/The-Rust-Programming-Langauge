//:: accesses the items of a module
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
        // =100 is inclusive
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");



    loop {
        println!("Guess the number!");
        println!("Please input your guess");
    
        // let is immutatable by default.
        // mut makes the variable mutable
        
        // The :: syntax in the ::new line indicates that new is an associated function of the String type. 
        // An associated function is a function that’s implemented on a type
        let mut guess = String::new();
    
        io::stdin()
            // Pass a reference of guess, and make it mutable 
            .read_line(&mut guess)
            // read_line returns a "Result", which has an Ok and Err.
            // If the result is an err, expect will cause the program to crash
            .expect("Failed to read line");
    
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // let guess creates a shadow of the String guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        // A match expression is made up of arms.
        // An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
        // Rust takes the value given to match and looks through each arm’s pattern in turn. 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
