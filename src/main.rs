use std::io;

fn main() {
    println!("Clothes Picker!");

    let mut input = String::new();
    println!("Enter the weather: ");
    io::stdin().read_line(&mut input).unwrap();
    let weather = input.trim();
    println!("Weather: {}", weather);
}
