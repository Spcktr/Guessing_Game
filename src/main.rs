extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // User greeting, and input field.
    println!("Guess the number.");
    // Generate random secret
    let secret_number = rand::thread_rng().gen_range(1, 10);
    // Call for the number and display
    println!("The secret number is: {}", secret_number);
    // loop for the game to restart under each guess
loop {
        println!("Please enter your guess.");
        // Setting input as immutable variable; input is string; setting variable to guess.
        let mut guess = String::new();
        // could also have been written as std::io::stdin; Handles user input
        io::stdin().read_line(&mut guess)
            .expect("Uh-oh, please enter your number again"); // Validates and returns fail text
        // converting string to rust integer and removing whitespaces
        let guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
            // echo user input
        println!("You guessed: {}", guess);
            // Compare input and random secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed under."),
            Ordering::Greater => println!("You guessed over."),
            Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }
}
