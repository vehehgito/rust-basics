use std::io;

fn main() {
    let mut number = String::new();
    println!("Enter a number to calculate the factorial of: ");
    io::stdin().read_line(&mut number).expect("Failed to read line");
    let number :u32 = number.trim().parse().expect("Please type a number!");

    let mut factorial = 1;
    for i in 1..number+1 {
        factorial *= i;
    }
    
    println!("The factorial of {} is {}.",number,factorial);
}
