use std::io;

fn main() {
    println!("EKEDC Smart Meter");
    println!("\nWelcome to the Smart Energy Company billing calculator!");

    println!("Note: N20.00 per KWh, above 100kWh at N25.00 per unit and above 200kWh at N30.00 per KWh");
    println!("Please enter your energy consumption (kWh):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let consumption: f64 = input.trim().parse().expect("Invalid input");

     let bill = if consumption <= 100.0 && consumption >= 0.0 {
        consumption * 20.0
    } else if consumption <= 200.0 && consumption > 100.0 {
        consumption * 25.0
    } else {
        consumption * 30.0
    };


    println!("Your total bill is: N{:.2}", bill);
}
