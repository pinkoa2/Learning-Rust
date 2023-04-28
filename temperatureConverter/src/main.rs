use std::io;

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn main() {
    println!("Temperature Converter...\nPlease input a temperature in Fahrenheit");

    let mut buf: String = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("Unable to read line");

    let buf = buf.trim();
    println!("Converting fahrenheit {buf} to celsius...");

    let fahrenheit: f32 = buf.trim().parse().expect("Unable to parse string to float");

    let celsius: f32 = fahrenheit_to_celsius(fahrenheit);

    println!("Finishing converting... in celsius the number is {celsius}")
}
