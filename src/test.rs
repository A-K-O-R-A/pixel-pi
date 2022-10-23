///Convert the pi.txt file to a binary file
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::str;

const BIN_PATH: &str = "./pi.bin";
const BLOCK_SIZE: usize = 1024 /16;

fn main() -> io::Result<()> {
    let mut bin_f = File::open(BIN_PATH)?;
    println!("opened binfile at {BIN_PATH}");
    
    
    let mut cur_buffer = [0; BLOCK_SIZE];
    let mut cur_pos: usize = 0;
    let file_length = bin_f.metadata().expect("Couldn't read length").len();

    while (cur_pos as u64) < file_length {
        //Seek forwards another block
        //f.seek(io::SeekFrom::Current(BLOCK_SIZE as i64))?;
        
        //Read one block into current buffer
        let _n = bin_f.read(&mut cur_buffer[..])?;

        //Convert bytes to numbers
        //println!("{:?}", cur_buffer);

        //println!("{} / {}", cur_pos/BLOCK_SIZE, file_length/(BLOCK_SIZE as u64));

        cur_pos += BLOCK_SIZE;

    }

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
        _ => 0
    }
}