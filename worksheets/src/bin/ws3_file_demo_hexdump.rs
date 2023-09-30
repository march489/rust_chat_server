static BUFFER_SIZE: usize = 16;
static MIN_CHARS_PER_LINE: i32 = 51;
static PERIOD_ASCII_CODE: u8 = 46;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn show_printable_chars(rch: &u8) -> u8 {
    let val = *rch;
    if val < 32 || val > 127 {
        PERIOD_ASCII_CODE
    } else {
        val
    }
}

fn hexdump(input: File) -> Result<(), Box<dyn Error>> {
    let mut address: usize = 0;
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, input);
    let mut printed_chars = 0;

    while reader.fill_buf()?.len() > 0 {
        print!("{:08x}: ", address);
        printed_chars += 10;

        for (index, ch) in reader.buffer().iter().enumerate() {
            print!("{:02x}", ch);
            printed_chars += 2;
            if 1 == index % 2 {
                print!(" ");
                printed_chars += 1;
            }

            if 7 == index {
                print!(" ");
                printed_chars += 1;
            }
        }

        // align right
        for _ in printed_chars..=MIN_CHARS_PER_LINE {
            print!(" ");
        }

        let vec_bytes: Vec<u8> = reader
            .buffer()
            .iter()
            .map(show_printable_chars)
            .collect::<Vec<u8>>();

        // print ascii representation
        print!("|{}|", String::from_utf8(vec_bytes)?);

        // reset for next cycle
        reader.consume(reader.buffer().len());
        address += BUFFER_SIZE;
        printed_chars = 0;
        println!();
    }

    Ok(()) // return that we're done
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let input_filename: &str = &args[0];
    let input: File = File::open(input_filename)?;
    hexdump(input) // spits out the result Ok()
}
