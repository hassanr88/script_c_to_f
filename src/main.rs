/*
By: Hassan Rana
Date: 2026-09-28
Program Details: Convert Fahrenheit to Celsius
*/
use::std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a temperature in Fahrenheit:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let temp = input.trim().parse::<f64>();
    if let Ok(temp) = (temp) {
        let celsius = (temp - 32.0) * 5.0 / 9.0;
        println!("The temperature in Celsius is: {}", celsius);
    }
    else {
        println!("Invalid input. Please enter a valid number.");
    }
}
