/*
    Guessing Game in RUST

    Program will ask for user input, process that input, and
    check that the input is in the expected form

    Grace Todd
*/

use std::io;

fn main() {
    println!("***************\nGuessing Game!\n***************");
    println!("Input your guess please");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Uh oh! Failed to read line");

    println!("Your guess: {guess}");
}
