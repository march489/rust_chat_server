use std::error::Error;
use std::io::Write;

fn main() -> Result<(), Box<dyn Error>> {
    println!("What is your name?");
    print!("> ");
    std::io::stdout().flush()?;
    let mut name: String = String::with_capacity(64);
    let mut age_response: String = String::with_capacity(32);

    let mut _num_bytes_read = std::io::stdin().read_line(&mut name)?;
    let trimmed_name: &str = name.trim();
    println!("Hello {trimmed_name}!");

    println!("What is your age?");
    print!("> ");
    std::io::stdout().flush()?;
    _num_bytes_read = std::io::stdin().read_line(&mut age_response)?;

    loop {
        match age_response.trim().parse::<u32>() {
            Err(_) => {
                eprintln!(
                    "Invalid input: [{}] cannot be parsed as a u32",
                    age_response.trim()
                );
                println!("What is your age?");
                print!("> ");
                std::io::stdout().flush()?;
                age_response.clear();
                _num_bytes_read = std::io::stdin().read_line(&mut age_response)?;
            }
            Ok(age) => {
                println!("You are {age} years old.");
                break;
            }
        }
    }

    println!("Goodbye {trimmed_name}!");
    Ok(())
}
