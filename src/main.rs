use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

mod util;

const PI_PATH: &str = "./pi_data/pi_dec_1b.txt";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let sequence = &args[1];
    println!("searching for sequence: {sequence:?}");

    let mut f = File::open(PI_PATH)?;
    f.seek(io::SeekFrom::Start(2))?; //Skip the first two characters "3."

    println!("opened pi text file at {PI_PATH:?}");

    // Only use printabel characters
    // Refer to https://www.ascii-code.com/
    /*
    let number_sequence = sequence
        .chars()
        .map(|c| (c as u8 - 32).to_string())
        .reduce(|acc, e| acc + &e)
        .unwrap();

    println!("number sequence: {number_sequence:?}");
    */

    let seq_pos = util::find_sequence(sequence.as_bytes(), &mut f)? + 1;
    println!("\n\n");
    println!("sequence found at position {seq_pos}");

    drop(f);
    println!("closed file");

    Ok(())
}
