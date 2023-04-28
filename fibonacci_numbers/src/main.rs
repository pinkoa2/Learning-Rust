use std::io;

fn fibonacci_sequence(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_sequence(n - 1) + fibonacci_sequence(n - 2),
    }
}

fn main() {
    let mut input: String = String::new();

    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Unable to read line");

    let input: u64 = input.trim().parse().expect("Unable to parse string to u64");

    println!("You have inputed {input}");

    let value: u64 = fibonacci_sequence(input);
    println!("The fibonacci value equals {value}");
}
