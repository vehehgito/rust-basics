use std::io;

fn main() {
    println!("This is a simple calculator");

    loop{
        println!("Enter an operator (+, -, *, /) or q to quit");
        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        let operator = operator.trim();

        if operator == "q"{
            break;
        }

        println!("Enter the first number");
    }
}
