use std::fs::File;
use std::io;
use std::io::prelude::*;

const BLOCK_SIZE: usize = 32;
/// Returns the offset at which point the provided sequence begins in the file (does not accountt for previous seeking)
pub fn find_sequence(seq: &[u8], f: &mut File) -> io::Result<usize> {
    let seq_size = seq.len();
    let file_size = f.metadata()?.len();

    let mut cur_buffer = [0; BLOCK_SIZE];
    let mut cur_offset: usize = 0;

    let _n = f.read(&mut cur_buffer[..seq_size])?;

    while (cur_offset as u64) <= file_size {
        // Read the next block into current buffer
        // Keep the old first (seq_size - 1) bytes to also find
        // Sequences that range over multiple blocks
        // n is the amount of bytes we read
        let n = f.read(&mut cur_buffer[seq_size..])?;

        // To account for our first (seq_size - 1) bytes that
        // we also search in we need to add them to the buffer range
        // that we are searching in
        if let Some((found_index, _)) = cur_buffer[0..(n + seq_size)]
            .windows(seq_size)
            .enumerate()
            .find(|(_i, w)| *w == seq)
        {
            return Ok(cur_offset + found_index);
        }

        // Add the last elements from the current buffer to
        // the beginning for the next buffer
        for i in 0..seq_size {
            cur_buffer[i] = cur_buffer[n + i];
        }

        cur_offset += BLOCK_SIZE;

        print!("\r{:.2}%", (cur_offset as f64) * 100.0 / (file_size as f64));
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Unable to find sequence in file",
    ))
}
