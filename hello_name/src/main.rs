use std::io;

fn main() {
    let mut name = String::new();

    println!("What is your name?");

    io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Hello, {}!", name);
}
