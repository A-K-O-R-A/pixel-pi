///Convert the pi.txt file to a binary file
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

const DEC_PATH: &str = "./pi_data/pi_dec_1b.txt";
const BLOCK_SIZE: usize = 1024 * 512;

fn main() -> io::Result<()> {
    let mut dec_f = File::open(DEC_PATH)?;
    let file_size = dec_f.metadata()?.len();
    dec_f.seek(io::SeekFrom::Start(2))?; //Skip the first two characters "3."
    println!("opened decimal file at {DEC_PATH}");

    let mut stats = vec![0u64; 10];

    // The first buffer that is going to be filled
    let mut cur_buffer = [0u8; BLOCK_SIZE];
    let mut cur_pos: usize = 0;

    while (cur_pos as u64) <= file_size {
        //Read block into buffer
        let n = dec_f.read(&mut cur_buffer[..])?;

        //Convert numbers to chars
        for c in &cur_buffer[0..n] {
            let num = char_to_num(*c as char);

            stats[num as usize] += 1;
        }

        print!("\r{:.2}%", (cur_pos as f64) * 100.0 / (file_size as f64));

        cur_pos += BLOCK_SIZE;
    }

    println!("\n");
    println!("closing files");
    drop(dec_f);

    println!("{:?}", stats);
    println!("Done");

    Ok(())
}

fn char_to_num(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => 0,
    }
}
