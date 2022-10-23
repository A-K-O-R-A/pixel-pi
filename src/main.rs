use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;
use std::env;

const FILE_PATH: &str = "./pi.txt";

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let sequence = &args[1];
    println!("sequence: {sequence}");

    let mut f = File::open(FILE_PATH)?;
    println!("opened text file {FILE_PATH}");

    let seq_pos = find_sequence(sequence, &mut f)?;
    println!("sequence found at position {seq_pos}");

    drop(f);
    println!("closed file");

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