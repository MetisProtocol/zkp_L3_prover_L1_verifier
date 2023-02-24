mod the_cpp_lib;
use the_cpp_lib::{init, volume, TheMemory};

fn main() {
    println!("Hello, world!");
    let mut some_memory = TheMemory::default();
    unsafe {
        let width = 9;
        let height = 10;
        let depth = 11;
        init(&mut some_memory, width, height);
        println!(
            "The volume is {} x {} x {} = {}",
            width,
            height,
            depth,
            volume(&mut some_memory, depth)
        );
    }
    println!("Just for fun, here are the bytes:");
    for byte in some_memory.opaque_memory.iter() {
        println!("{:x}", byte);
    }
}
