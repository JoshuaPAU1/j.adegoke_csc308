
//writing a prgoram that uses a simple closure to filter even numbers from a vector containing the first 20 natural numbers

fn main(){
    let numbers: Vec<i32> = (1..=20).collect();

    let even = |x: &i32| x % 2 == 0;

    let even_numbers: Vec<i32> = numbers.iter().cloned().filter(even).collect();

    println!("Even numbers: {:?}", even_numbers);
}