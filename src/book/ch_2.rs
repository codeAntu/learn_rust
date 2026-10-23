use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

pub fn run() {
    let hidden_number = rand::rng().random_range(1..=100);

    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&hidden_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}
