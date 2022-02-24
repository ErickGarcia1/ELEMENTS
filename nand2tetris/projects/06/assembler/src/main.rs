use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("testfile.asm").unwrap(); //.expect("Can't open file!");

    // let mut contents = String::new();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        
        println!("{}\t{}", index + 1, line);
    }

    // println!("File contents:\n\n{}",contents);

}
