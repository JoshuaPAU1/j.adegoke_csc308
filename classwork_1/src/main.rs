use std::io;

fn main() {

    println!("Temperature Converter");

    println!("\nPlease enter your temperature in Celsius: ");
    let mut temp1 = String::new();
    io::stdin().read_line(&mut temp1).expect("Failed to read line");

    println!("\nPlease enter your temperature in Fahrenheit: ");
    let mut temp2 = String::new();
    io::stdin().read_line(&mut temp2).expect("Failed to read line");

    let temp2:f64 = temp2.trim().parse().expect("Input");


    let temp1:f64 = temp1.trim().parse().expect("Input");

    let fh:f64 = ((9.0/5.0) * temp1) + 32.0 ;
    println!("Your temperature in Fahrenheit is: {:.2}F", fh);

    let cels:f64 = (temp2 - 32.0) * 5.0/9.0 ; 
    println!("Your temperature in Celsius is: {:.2}C", cels);
    