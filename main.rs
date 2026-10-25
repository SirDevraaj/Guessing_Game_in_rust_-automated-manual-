mod automated;
mod game;

use colored::*;
use std::io::{self, Write};

fn main() {
    println!("{}", "HEYYYYYYYYY WELCOME TO THE GAME!!!!!!!".cyan().bold());

    println!();
    println!("Choose your game mode:");
    println!("1. Manual");
    println!("2. Automated");
    println!("3. Exit");

    loop {
        print!("\nEnter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input.");

        match choice.trim() {
            "1" => {
                game::manual_game();
                break;
            }

            "2" => {
                automated::automated_game();
                break;
            }

            "3" => {
                println!("{}", "Goodbye!".cyan());
                break;
            }

            _ => {
                println!("{}", "Invalid choice. Enter 1, 2, or 3.".red());
            }
        }
    }
}
