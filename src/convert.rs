///Convert the pi.txt file to a binary file
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

const DEC_PATH: &str = "./pi_data/pi_hex_1b.txt";
const BIN_PATH: &str = "./pi_data/pi_bin_1b.txt";
const BLOCK_SIZE: usize = 1024 * 512;

fn main() -> io::Result<()> {
    let mut hex_f = File::open(DEC_PATH)?;
    let file_size = hex_f.metadata()?.len();
    hex_f.seek(io::SeekFrom::Start(2))?; //Skip the first two characters "3."
    println!("opened hexadecimal file at {DEC_PATH}");

    let mut bin_f = File::create(BIN_PATH)?;
    println!("created bin file at {BIN_PATH}");

    // The first buffer that is going to be filled
    let mut cur_buffer = [0u8; BLOCK_SIZE];
    let mut cur_pos: usize = 0;

    while (cur_pos as u64) <= file_size {
        //Read block into buffer
        let n = hex_f.read(&mut cur_buffer[..])?;

        //Convert numbers to chars
        let bytes: Vec<u8> = cur_buffer[0..n]
            .chunks(2)
            .map(|chunk| char_to_hexnum(chunk[0]) | char_to_hexnum(chunk[0]) << 4)
            .collect();
        bin_f.write(&bytes)?;

        print!("\r{:.2}%", (cur_pos as f64) * 100.0 / (file_size as f64));

        cur_pos += BLOCK_SIZE;
    }

    println!("\n");
    println!("closing files");
    drop(hex_f);
    drop(bin_f);
    println!("Done");

    Ok(())
}

fn char_to_hexnum(c: u8) -> u8 {
    match c as char {
        '0' => 0x0,
        '1' => 0x1,
        '2' => 0x2,
        '3' => 0x3,
        '4' => 0x4,
        '5' => 0x5,
        '6' => 0x6,
        '7' => 0x7,
        '8' => 0x8,
        '9' => 0x9,
        'a' => 0xa,
        'b' => 0xb,
        'c' => 0xc,
        'd' => 0xd,
        'e' => 0xe,
        'f' => 0xf,
        _ => 0,
    }
}
