use std::io;


fn number_input(msg: &str) -> i32{
    loop {
        let mut input = String::new();
        println!("{} integer:", msg);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(num) => return num, // Valid integer, return it
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

fn string_input() -> char {
    loop {
        let mut input = String::new();

        println!("Enter operation to perform (+, -, *, /):");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let trimmed = input.trim();

        // Ensure the input is a single character and one of the valid operations
        if trimmed.len() == 1 {
            let ch = trimmed.chars().next().unwrap();
            if "+-*/".contains(ch) {
                println!("{}", ch);
                return ch; // Return valid character
            }
        }

        // If input is invalid, print an error and prompt again
        println!("Error: Please enter one of the valid operations (+, -, *, /).");
    }
}

fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn sub_two_numbers(a: i32, b: i32) -> i32 {
    a - b
}

fn mul_two_numbers(a: i32, b: i32) -> i32 {
    a * b
}

fn divide_two_numbers(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    let value1 = number_input("First");
    let value2 = number_input("Second");
    let op:char =string_input();

    let sum=add_two_numbers(value1, value2);
    let substract=sub_two_numbers(value1, value2);
    let mul: i32=mul_two_numbers(value1, value2);
    let div: i32 = divide_two_numbers(value1, value2);

    match op {
        '+' => println!("The sum is {}", sum),
        '-' => println!("The sub is {}", substract),
        '/' => println!("The division is {}", div),
        '*' => println!("The multiply is {}", mul), // Match multiple values
        _ => println!("Something else!"),  // Default case
    }
}
