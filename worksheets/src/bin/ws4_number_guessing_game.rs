use rand::Rng;
use std::{error::Error, io::Write};

const UPPER_LIMIT: u32 = 1000;

fn prompt_user() -> Result<(), std::io::Error> {
    println!("Enter a number between 0 and {UPPER_LIMIT}");
    print!("> ");
    std::io::stdout().flush()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let mut _guess: u32 = 0;
    let mut response = String::with_capacity(16);
    let magic_number: u32 = rng.gen_range(0..UPPER_LIMIT);

    loop {
        response.clear();
        prompt_user()?;
        match std::io::stdin().read_line(&mut response) {
            Err(e) => {
                eprintln!("Fatal error: {e}");
                panic!();
            }

            Ok(_) => { /* do nothing */ }
        }

        match response.trim().parse::<u32>() {
            Ok(num) => _guess = num,
            Err(_) => {
                eprintln!(
                    "Invalid input: [{}] cannot be parsed as a u32",
                    response.trim()
                );
                continue;
            }
        }

        if _guess < magic_number {
            println!("Your guess is too small!");
        } else if _guess > magic_number {
            println!("Your guess is too big!");
        } else {
            println!("You guessed correctly!");
            break;
        }
    }

    Ok(())
}
