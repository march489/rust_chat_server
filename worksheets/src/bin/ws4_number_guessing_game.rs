use rand::Rng;
use std::{error::Error, io::Write};

const UPPER_LIMIT: u32 = 100;

fn get_user_guess() -> Result<(), std::io::Error> {
    println!("Enter a number between 0 and {UPPER_LIMIT}");
    print!("> ");
    std::io::stdout().flush()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut response = String::with_capacity(16);
    let magic_number: u32 = rng.gen_range(0..UPPER_LIMIT);

    loop {
        response.clear();
        get_user_guess()?;
        std::io::stdin().read_line(&mut response)?;

        match response.trim().parse::<u32>() {
            Ok(guess) => {
                if guess < magic_number {
                    println!("Your guess is too small!");
                } else if guess > magic_number {
                    println!("Your guess is too big!");
                } else {
                    println!("You guessed correctly!");
                    break;
                }
            }
            Err(_) => {
                eprintln!(
                    "Invalid input: [{}] cannot be parsed as a u32",
                    response.trim()
                );
            }
        }
    }

    Ok(())
}
