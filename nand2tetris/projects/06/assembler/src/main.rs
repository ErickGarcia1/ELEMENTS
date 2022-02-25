use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let mut symbol_table= HashMap::from([
        ("R0", 0),
        ("R1", 1),
        ("R2", 2),
        ("R3", 3),
        ("R4", 4),
        ("R5", 5),
        ("R6", 6),
        ("R7", 7),
        ("R8", 8),
        ("R9", 9),
        ("R10", 10),
        ("R11", 11),
        ("R12", 12),
        ("R13", 13),
        ("R14", 14),
        ("R15", 15),
        ("SCREEN", 16384),
        ("KBD", 24576),
        ("SP", 0),
        ("LCL", 1),
        ("ARG", 2),
        ("THIS", 3),
        ("THAT", 4),
        ("JGT", 1),
        ("JEQ", 2),
        ("JGE", 3),
        ("JLT", 4),
        ("JNE", 5),
        ("JLE", 6),
        ("JMP", 7),
    ]);

    // println!("{}", symbol_table.entry("R15"));
    for (symbol, address) in &symbol_table {
        println!("{} -> {}", symbol, address);
    }

    let s = String::from("foo bar");
    // s.remove_matches(" ");
    println!("{}", s);
    // assert_eq!("foo", s.as_str());

    let file = File::open("testfile.asm").unwrap(); //.expect("Can't open file!");

    // let mut contents = String::new();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let mut line = line.unwrap();
        if line.contains("//") {
            println!("Comment");
            let mut slashes: (&str, &str) = line.split("//").collect();
            line = slashes.first();
            println!("split output: {:?}",slashes);
        } else {
            println!("No comment");
        }
        
        println!("{}\t{}", index + 1, line);
    }

    // println!("File contents:\n\n{}",contents);

}
