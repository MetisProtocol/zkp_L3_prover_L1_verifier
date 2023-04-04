use std::fs::File;
use std::io::{BufReader, Read, ErrorKind};

pub fn read_file(filename: &String) -> Vec<u32> {
    let file = File::open(filename).expect("Failed to open file.");
    let mut reader = BufReader::new(file);

    let mut instructions = Vec::<u32>::new();
    loop {
        let mut buffer = [0_u8; std::mem::size_of::<u32>()];
        let read_result = reader.read_exact(&mut buffer);
        match read_result {
            Err(error) if error.kind() == ErrorKind::UnexpectedEof => break,
            _ => {},
        }
        read_result.expect("Unexpected error during read");
        let f = u32::from_be_bytes(buffer);
        instructions.push(f);
    }
    instructions
}
