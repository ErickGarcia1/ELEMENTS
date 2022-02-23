use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("testfile.asm").expect("Can't open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Cannot read file :(");

    println!("File contents:\n\n{}",contents);

}
