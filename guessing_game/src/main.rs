/*
    First real Rust program
    Austin Harshberger, 2022
    Free and Open source
    Reference: doc.rust-lang.org/book/ch02-00-guessting-game-tutorial.html
*/


// import the standard library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// define the standar main function that is used and first processed by Rust
fn main() {
    // print the string "Guess the number!" to stdout
    println!("\nLet's play the game: \"Guess the number!\"\n");
    
    // create a variable that is assigned to the output of the rand function
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // print the string "The secret number is" followed by the value stored in `secret_number`
    // println!("The secret number is: {secret_number}");

    // create a loop
    loop {
        // print the string "Plesee input your guess." to stdout
        println!("Please input your guess.");

        // create a variable called "guess" to store user input
        let mut guess = String::new();
        
        // call the io module from the io library include in use std::io to aaccept user input
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // convert their guess to an integeer vlue so it can be compared to the random number generated
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("The value you entered was not a number. Try again!");
                continue;
            }
        };
        
        // print the user acceepted inpit `guess` after "You guessed: " to stdout
        println!("You guessed: {guess}");
        
        // compare the user entered value and the random number generate value using the cmp module from std:cmp::Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        } 
    }   
}

// EOF