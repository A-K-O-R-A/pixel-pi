use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

const FILE_PATH: &str = "./pi.txt";
const SEQUENCE: &str = "6942000";

fn main() -> io::Result<()> {
    let mut f = File::open(FILE_PATH)?;
    println!("text file {FILE_PATH}");
    println!("sequence {SEQUENCE}");
    let seq_pos = find_sequence(SEQUENCE, &mut f)?;
    println!("sequence found at position {seq_pos}");

    Ok(())
}

const BLOCK_SIZE: usize = 1024 / 2;

fn find_sequence(str_seq: &str, f: &mut File) -> io::Result<usize> {
    let seq: &[u8] = str_seq.as_bytes();
    let seq_len = seq.into_iter().count();

    let mut cur_buffer = [0; BLOCK_SIZE];
    let mut cur_pos: usize = 0;
    let file_length = f.metadata()?.len();

    while (cur_pos as u64) <= file_length {
        //Read one block into current buffer
        let _n = f.read(&mut cur_buffer[..])?;

        for i in 0..(BLOCK_SIZE - seq_len) {
            let buf_slice = &cur_buffer[i..(i + seq_len)];
            //Search sequence in specific Block
            let same = seq == buf_slice;
            if same {
                return Ok(cur_pos+i - 2);
            }
        }


        cur_pos += BLOCK_SIZE;
    }

    Err(io::Error::new(io::ErrorKind::Other, "Didn't find sequence"))
}