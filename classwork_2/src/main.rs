use std::io;

fn main() {
    


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
