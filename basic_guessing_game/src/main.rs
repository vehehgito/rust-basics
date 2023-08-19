use std::io;

fn main() {
    let mut guess = String::new();
    let answer = 298;

    println!("Guess the three-digit number:");
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    if guess == answer {
        println!("You win!!")
    }
    else {
        println!("You lose!!")
    }
}
