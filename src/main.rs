use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

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
    let number_sequence = sequence
        .chars()
        .map(|c| (c as u8 - 32).to_string())
        .reduce(|acc, e| acc + &e)
        .unwrap();

    println!("number sequence: {number_sequence:?}");

    let seq_pos = find_sequence(sequence.as_bytes(), &mut f)? + 1;
    println!("\n\n");
    println!("sequence found at position {seq_pos}");

    drop(f);
    println!("closed file");

    Ok(())
}

const BLOCK_SIZE: usize = 1024 * 512;

/// Returns the offset at which point the provided sequence begins in the file (does not accountt for previous seeking)
fn find_sequence(seq: &[u8], f: &mut File) -> io::Result<usize> {
    let seq_len = seq.len();

    let mut cur_buffer = [0; BLOCK_SIZE];
    let mut cur_offset: usize = 0;
    let file_size = f.metadata()?.len();

    while (cur_offset as u64) <= file_size {
        //Read one block into current buffer
        let n = f.read(&mut cur_buffer[..])?;

        if let Some((found_index, _)) = cur_buffer[0..n]
            .windows(seq_len)
            .enumerate()
            .find(|(_i, w)| *w == seq)
        {
            return Ok(cur_offset + found_index);
        }

        cur_offset += BLOCK_SIZE;

        print!("\r{:.2}%", (cur_offset as f64) * 100.0 / (file_size as f64));
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Unable to find sequence in file",
    ))
}
