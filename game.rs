use colored::*;
use rand::RngExt;
use std::cmp::Ordering;
use std::io::{self, Write};

const MAX_NUMBER: u32 = 99_999;

pub fn manual_game() {
    println!("{}", "\n=== MANUAL MODE ===".cyan());
    println!("Guess the secret number between 0 and {MAX_NUMBER}.");

    let secret_number = rand::rng().random_range(0..=MAX_NUMBER);
    let mut attempts = 0;

    loop {
        let mut input = String::new();

        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");

        let guess: u32 = match input.trim().parse() {
            Ok(num) if num <= MAX_NUMBER => num,
            _ => {
                println!(
                    "{}",
                    format!("Enter a number between 0 and {MAX_NUMBER}.").red()
                );
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small!".yellow());
            }

            Ordering::Greater => {
                println!("{}", "Too big!".yellow());
            }

            Ordering::Equal => {
                println!(
                    "{}",
                    format!("🎉 Correct! {guess} was the number. Attempts: {attempts}").green()
                );
                break;
            }
        }
    }
}
