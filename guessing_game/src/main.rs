
extern crate rand; //ets Rust know we’ll be using that external dependency

use std::io;
use std::cmp::Ordering;
use rand::Rng; //Rng is a trait that defines methods that random number generators implement, and this trait must be in scope for us to use those methods


fn main() {
    //println!("Hello, world!");

    println!("Guess the number");


    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number );
    loop{
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");



    // let guess: u32 = guess.trim().parse()
    //     .expect("Please type a number!");
    let guess: u32 = match guess.trim().parse() {
        Ok(num)    => num,
        Err(_)    => continue,
    };
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
    	Ordering::Less    => println!("Too small!"),
    	Ordering::Greater => println!("Too big!"),
    	Ordering::Equal   => {
    		println!("You win!");
    		break;
    	}


    }
}
    
}
