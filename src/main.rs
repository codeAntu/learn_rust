use rand::Rng;
use std::io;
use colored::*;

fn main() {
    let hidden_number = rand::rng().random_range(1..=100);

    loop {
        let mut guess: String = String::new();
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&hidden_number) {
            std::cmp::Ordering::Less => println!("Too small!".red()),
            std::cmp::Ordering::Greater => println!("Too big!".red()),
            std::cmp::Ordering::Equal => {
                println!("You win!".green());
                break;
            }
        }
    }
}
