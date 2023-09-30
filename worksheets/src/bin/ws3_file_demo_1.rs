use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let input_filename: &str = &args[0];
    let data: String = std::fs::read_to_string(input_filename)?;
    println!("{}", data);

    // prints the number of characters
    println!("chars in file [{}]: {}", input_filename, data.len());

    println!("-------------- reversed file ---------------");

    // prints the file in reverse
    let mut reversed_data = data.clone().chars().collect::<Vec<char>>();
    reversed_data.reverse();
    println!(
        "reversed file: \n{} \n",
        reversed_data.iter().collect::<String>()
    );

    // prints the file between >> ... << chevrons
    println!("--------------- >> tac << ---------------");
    for line in data.lines().rev() {
        println!(">> {} << [num chars: {}]", line, line.len());
    }

    // keeps the order, but reverses each line
    println!("---------- same order, individual lines reversed ---------");
    for line in data.lines() {
        let newline = line.chars().rev().collect::<String>();
        println!("{}", newline);
    }

    Ok(())
}
