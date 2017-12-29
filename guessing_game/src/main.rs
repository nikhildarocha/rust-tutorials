
extern crate rand; //ets Rust know we’ll be using that external dependency

use std::io;
use rand::Rng; //Rng is a trait that defines methods that random number generators implement, and this trait must be in scope for us to use those methods


fn main() {
    //println!("Hello, world!");

    println!("Guess the number");


    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number );

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");



    println!("You guessed: {}", guess);

}
