use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut guess: String;
    let secret_number: i32 = rand::thread_rng().gen_range(1..=1000);

    loop {
        println!("Input a guess: ");

        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read from stdin");

        let num: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Unable to parse string to int");
                continue;
            }
        };

        match num.cmp(&secret_number) {
            Ordering::Less => println!("Guess is too low"),
            Ordering::Greater => println!("Guess is too high"),
            Ordering::Equal => {
                println!("You guessed the correct number");
                break;
            }
        }
    }

    println!("Done")
}
