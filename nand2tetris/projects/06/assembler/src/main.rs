use std::fs::OpenOptions;
use std::io::{Write, BufRead, BufReader};
use std::collections::HashMap;
use std::path::Path;

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

    // Open .asm file and create a .hack file with the same name
    let mut file_name = String::from("testfile.asm");

    let file_in = {
        OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&file_name)
        .expect(&format!("Could not open {}!", &file_name))
    };

    println!("Opening {}...", &file_name);

    file_name = file_name.replace(".asm", ".hack");

    let mut file_out =  {
        OpenOptions::new()
        .read(false)
        .write(true)
        .create(true)
        .open(&file_name)
        .expect(&format!("Could not create or open {}!", &file_name))
    };
    

    // Create a reading buffer for input file
    let reader = BufReader::new(file_in);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains("//") {
            println!("Comment");
            let slashes: Vec<&str> = line.split("//").collect();
            let new_line = slashes[0].to_string();
            if !new_line.is_empty() {
                // write!(&file_out, "Bruh\n");
            }

        }
        
        println!("{}\t{}", index + 1, line);
    }

    // println!("File contents:\n\n{}",contents);

}
