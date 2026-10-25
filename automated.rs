use colored::*;
use rand::RngExt;
use std::cmp::Ordering;

const MAX_NUMBER: u32 = 99_999;

pub fn automated_game() {
    println!("{}", "\n=== AUTOMATED MODE ===".cyan());

    let secret_number = rand::rng().random_range(0..=MAX_NUMBER);

    println!("Computer is searching for the secret number...");
    println!("Search range: 0 - {MAX_NUMBER}\n");

    let mut low = 0;
    let mut high = MAX_NUMBER;
    let mut attempts = 0;

    while low <= high {
        let mid = low + (high - low) / 2;

        attempts += 1;

        println!("Attempt {attempts}: Computer guesses {mid}");

        match mid.cmp(&secret_number) {
            Ordering::Less => {
                println!("{} {mid} is too small.\n", "→".yellow());

                low = mid + 1;
            }

            Ordering::Greater => {
                println!("{} {mid} is too big.\n", "→".yellow());

                high = mid - 1;
            }

            Ordering::Equal => {
                println!(
                    "{}",
                    format!(
                        "🎯 Found it! The number was {secret_number}."
                    )
                    .green()
                );

                println!("Binary search took {attempts} attempts.");
                break;
            }
        }
    }
}
