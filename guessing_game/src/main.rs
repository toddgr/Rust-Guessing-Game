/*
    Guessing Game in RUST

    Program will ask for user input, process that input, and
    check that the input is in the expected form

    Grace Todd
*/

use std::io;
use rand::Rng;

fn process_guess() -> String { 

    println!("Input your guess please");

    // Since we cannot really know what the user input will be, this variable is mutable
    let mut guess = String::new();

    // Read in line, handle potential failure of unexpected input. Expect is required
    io::stdin()
        .read_line(&mut guess)
        .expect("Uh oh! Failed to read line");

    println!("Your guess: {guess}");

    guess
}

fn generate_secret_number() -> u32 { 
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Psst... the secret number is {secret_number}");
    
    secret_number
}

fn compare(guess, secret_number) {
    
}


fn main() {
    println!("***************\nGuessing Game!\n***************");

    let secret_number = generate_secret_number();
    let guess = process_guess();

    compare(guess, secret_number);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_guess() {
        assert_eq!(3,3)
    }

    #[test]
    fn test_generate_secret_number() {
        assert_eq!(4,4)
    }
}