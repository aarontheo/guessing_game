use rand::prelude::*;
use std::{io::{self, stdin, Write}, num::ParseIntError};

// struct NonNegNonZeroInt(i32);

// enum NNNZCreationError {
//     ZeroError,
//     NegativeError
// }

// impl NonNegNonZeroInt {
//     fn new(num :i32) -> Result<NonNegNonZeroInt, NNNZCreationError> {
//         if num > 0 {
//             Ok(NonNegNonZeroInt(num))
//         } else if num == 0 {
//             Err(NNNZCreationError::ZeroError)
//         } else {
//             Err(NNNZCreationError::NegativeError)
//         }
//     }
// }

fn read_number() -> Result<i32, ParseIntError> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Error reading from stdin");
    buffer
    .trim()
    .parse::<i32>()
}

fn main() {
    println!("\n\n\nWelcome to Guess the Number!™");
    println!("I'll come up with a random number from 1 to 10, you'll guess, 
and I'll tell you if your guess was higher or lower.\n");

    let mut rng = rand::thread_rng();
    let secret_number :i32 = rng.gen_range(1..=10);
    let mut current_guess = 0;
    let mut spelling_errors:u16 = 0;
    let mut guesses:u16 = 0;

    println!("Alright, I've got the number!");
    //until the user guesses correctly,
    while current_guess != secret_number {
        //make sure that the thing the user types is actually an integer
        current_guess = loop {
            print!("Enter a number: ");
            io::stdout().flush().expect("Failure to... flush? stdout's clogged.");
            match read_number() {
                Ok(n) => {
                    guesses += 1;
                    break n
                },
                Err(_) => {
                    spelling_errors += 1;
                    println!("That's not an integer. Try again!");
                },
            }
        };
        if current_guess < secret_number {
            println!("{} is too low. Try again!", current_guess)
        } else if current_guess > secret_number {
            println!("{} is too high. Try again!", current_guess)
        }
    }
    println!("You guessed it! The number was {}.", secret_number);
    if spelling_errors == 0 {
        println!("It only took you {} guesses.", guesses)
    } else {
        println!("It only took you {} guesses, {} if you count the mistakes you've made.", guesses, guesses + spelling_errors);
    }
}
