use std::io;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let answer = rng.gen_range(0..101);

    let mut guess = String::new();
    println!("Guess the number between 1-100");
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    if guess == answer{
        println!("You win! The number was {}.",answer);
    }

    else {
        println!("You lose! The number was {}.",answer);
    }

    
    
}
