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
    println!("Your temperature in Fahrenheit is: {}F", fh);

    let cels:f64 = (temp2 - 32.0) * 5.0/9.0 ; 
    println!("Your temperature in Fahrenheit is: {}C", cels);
=======
    println!("Welcome to Smart Cafe");
    println!("1 - Black coffee - N7000 \n2 - Decaf - N1000 \n3 - Tea - N1500 \n4 - Bread - N3000");


    println!("\nPlease enter your order number: ");
    let mut order = String::new();
    io::stdin().read_line(&mut order).expect("Failed to read line");

    if order.trim() == "1" {
        order = "7000".to_string();
    } else if order.trim() == "2" {
        order = "1000".to_string();
    } else if order.trim() == "3" {
        order = "1500".to_string();
    } else if order.trim() == "4" {
        order = "3000".to_string();
    } else {
        println!("Invalid input");
        return;
    }


    println!("\nPlease enter the quantity: ");
    let mut quant = String::new();
    io::stdin().read_line(&mut quant).expect("Failed to read line");

    let quant:i64 = quant.trim().parse().expect("Input");


    let order:i64 = order.trim().parse().expect("Input");

    let amount:i64 = (quant * order);
    println!("Your total amount is: N{}", amount);

    if amount > 5000 && amount < 10000{
        let discount = amount as f64 * 0.1;
        let new_amount = amount as f64 - discount;
        println!("You get a 5% discount of N{}. Your new total amount is: N{}", discount, new_amount);

    } else if amount >= 10000 {
        let discount = amount as f64 * 0.15;
        let new_amount = amount as f64 - discount;
        println!("You get a 10% discount of N{}. Your new total amount is: N{}", discount, new_amount);
    } else {
        println!("No discount applied. Your total amount is: N{}", amount);
    }


}
