use std::io;

fn main() {
    println!("This is a simple calculator :)");

    loop{
        println!("Enter an operator (+, -, *, /, %) or q to quit:");
        let mut operator = String::new();
        io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line");
        let operator = operator.trim();

        if operator == "q"{
            break;
        }

        println!("Enter the first number:");
        let mut first_number = String::new();
        io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read line");
        let first_number : u32 = first_number.trim().parse().expect("Please type a number!");

        println!("Enter the second number:");
        let mut second_number = String::new();
        io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read line");
        let second_number : u32 = second_number.trim().parse().expect("Please type a number!");

        if operator == "+"{
            println!("{} + {} = {}", first_number, second_number, first_number + second_number);
        }
        else if operator == "-"{
            println!("{} - {} = {}", first_number, second_number, first_number - second_number);
        }
        else if operator == "*"{
            println!("{} * {} = {}", first_number, second_number, first_number * second_number);
        }
        else if operator == "/"{
            println!("{} / {} = {}", first_number, second_number, first_number / second_number);
        }
        else if operator == "%"{
            println!("{} % {} = {}", first_number, second_number, first_number % second_number);
        }
        else{
            println!("Invalid operator");
        }
    
    }
}
