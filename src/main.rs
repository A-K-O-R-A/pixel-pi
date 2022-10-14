use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::str;
//use std::io::SeekFrom;

fn main() -> io::Result<()> {
    let file_path = "./pi.txt";
    let mut f = File::open(file_path)?;
    println!("PI file {}", file_path);

    let seq = find_sequence(&[3, 1, 4], &mut f);
    println!("Seq: {:?}", seq);

    Ok(())
}

const BLOCK_SIZE: usize = 1024 / 2;

fn find_sequence(str_seq: &str, f: &mut File) -> io::Result<()> {
    let mut buffer = [0; BLOCK_SIZE];
    let seq: &[u8] = str_seq.as_bytes();
  
    //Every digit is 2 bytes
    let pos: u64 = 0;
    f.seek(io::SeekFrom::Start(pos))?;
    // up to one block
    let n = f.read(&mut buffer[..])?;
    buffer.
    println!("Buffer len: {n}");

    let s = str::from_utf8(&buffer).expect("L");
    println!("Buffer: {s}");
    //Search squenze in specific Block

    //Seek to next block -> redo until found
    Ok(())
}
