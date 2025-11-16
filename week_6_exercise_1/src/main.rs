use std::io;

fn main() {

    println!("Enter your number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: i64 = input.trim().parse().expect("Invalid input");


    let factorial = |x: i64| -> i64 {
        let mut result = 1;
        for i in 1..=x {
            result *= i;
        }
        result
    };

    println!("Factorial of {} is {}", input, factorial(input));
}
