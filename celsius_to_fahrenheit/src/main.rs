use std::io;


fn main() {
    println!("Please input a temperature in Celsius:");
    let celsius = read_float();
    let fahrenheit = convert_celsius_to_fahrenheit(celsius);
    println!("{celsius}°C is {fahrenheit}°F.");
}


fn read_float() -> f64 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    input.trim().parse().expect("Not a number!")
}


fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}
