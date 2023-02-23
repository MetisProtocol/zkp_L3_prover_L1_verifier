use std::io;

pub fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    input.trim().parse::<i32>().expect("Input was not 4 bytes.")
}