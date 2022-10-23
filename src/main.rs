use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;

const FILE_PATH: &str = "./pi.txt";
const SEQUENCE: &str = "694208008";

fn main() -> io::Result<()> {
    let mut f = File::open(FILE_PATH)?;
    println!("PI file:  \"{}\"", FILE_PATH);
    println!("Sequence: \"{}\"", SEQUENCE);
    let seq_pos = find_sequence(SEQUENCE, &mut f).expect("Sequence found");
    println!("Sequence found at position {seq_pos}");

    Ok(())
}

const BLOCK_SIZE: usize = 1024 / 2;

fn find_sequence(str_seq: &str, f: &mut File) -> io::Result<usize> {
    let mut buffer = [0; BLOCK_SIZE];
    let seq: &[u8] = str_seq.as_bytes();
    let seq_len = seq.into_iter().count();

    //Every digit is 2 bytes
    let mut pos: usize = 0;

    loop {
        f.seek(io::SeekFrom::Start(pos as u64))?;
        // up to one block
        let _n = f.read(&mut buffer[..])?;
        //buffer.
        //println!("Buffer len: {n}");

        for i in 0..(BLOCK_SIZE - seq_len) {
            let buf_slice = &buffer[i..(i + seq_len)];
            //let sli = str::from_utf8().expect("L");
            //Search squenze in specific Block
            let same = seq == buf_slice;
            if same {
                return Ok(pos+i);
            }
        }

        //let s = str::from_utf8(&buffer).expect("L");
        //println!("Buffer: {s}");

        //Seek to next block -> redo until found
        pos += BLOCK_SIZE;
    }
}
