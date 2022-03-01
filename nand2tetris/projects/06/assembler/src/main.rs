use std::{fs, env};
use std::io::{Seek, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct Command {
    line: String,
    c_type: CommandType,
    symbol: String,
    dest: String,
    comp: String,
    jump: String
}

struct Tables {
    symbol_table: HashMap<String, i32>,
    lookup_table: HashMap<String, i32>,
    comp_table: HashMap<String, i32>,
}

impl Command {
    fn new(line: &String) -> Command {
        // Create a new command object
        let mut out_command = Command {
            line: line.clone(),
            c_type: Command::command_type(line),
            symbol: String::new(),
            dest: String::new(),
            comp: String::new(),
            jump: String::new()
        };

        // If command is C type, parse the command. If it is
        // A or L type, parse it to get the symbol
        if !(out_command.c_type == CommandType::CCommand) {
            Command::symbol(&mut out_command);
        } else {
            Command::parse_command(&mut out_command);
        }
        out_command
    }

    fn command_type(line: &String) -> CommandType {
        // Return the command type of the current command
        if line.contains("@") {CommandType::ACommand} 
        else if line.contains("=") || line.contains(";") {CommandType::CCommand} 
        else {CommandType::LCommand}
    }

    fn symbol(command: &mut Command) {
        // Extract the symbol in the current line
        if command.c_type == CommandType::ACommand {
            let line_split: Vec<&str> = command.line.split("@").collect();
            command.symbol = line_split[1].to_string();
        } else if command.c_type == CommandType::LCommand {
            let mut line_split: Vec<&str> = command.line.split("(").collect();
            line_split = line_split[1].split(")").collect();
            command.symbol = line_split[0].to_string();
        }
    }
    
    fn parse_command(command: &mut Command) {
        let mut splitter = "";

        // Set the value that the string will be split on depending on the makeup of the line 
        if command.line.contains("=") {splitter = "=";} 
        else if command.line.contains(";") {splitter = ";";}

        let line_split: Vec<&str> = command.line.split(splitter).collect();
        // command.dest = line_split[0].to_string();
        if command.line.contains("=") {
            command.dest = line_split[0].to_string();
            command.comp = line_split[1].to_string();
        } else if command.line.contains(";") {
            command.comp = line_split[0].to_string();
            command.jump = line_split[1].to_string();
        }
    }
}

impl Tables {
    fn new() -> Tables {
        let tables = Tables {
            symbol_table: HashMap::from([
                (String::from("R0"), 0),
                (String::from("R1"), 1),
                (String::from("R2"), 2),
                (String::from("R3"), 3),
                (String::from("R4"), 4),
                (String::from("R5"), 5),
                (String::from("R6"), 6),
                (String::from("R7"), 7),
                (String::from("R8"), 8),
                (String::from("R9"), 9),
                (String::from("R10"), 10),
                (String::from("R11"), 11),
                (String::from("R12"), 12),
                (String::from("R13"), 13),
                (String::from("R14"), 14),
                (String::from("R15"), 15),
                (String::from("SCREEN"), 16384),
                (String::from("KBD"), 24576),
                (String::from("SP"), 0),
                (String::from("LCL"), 1),
                (String::from("ARG"), 2),
                (String::from("THIS"), 3),
                (String::from("THAT"), 4),
            ]),
            lookup_table: HashMap::from([
               (String::from(""),    0b000),
               (String::from("0"),    0b000),
               (String::from("JGT"), 0b001),
               (String::from("JEQ"), 0b010),
               (String::from("JGE"), 0b011),
               (String::from("JLT"), 0b100),
               (String::from("JNE"), 0b101),
               (String::from("JLE"), 0b110),
               (String::from("JMP"), 0b111),
               (String::from("M"),   0b001),
               (String::from("D"),   0b010),
               (String::from("MD"),  0b011),
               (String::from("DM"),  0b011),
               (String::from("A"),   0b100),
               (String::from("AM"),  0b101),
               (String::from("MA"),  0b101),
               (String::from("AD"),  0b110),
               (String::from("DA"),  0b110),
               (String::from("AMD"), 0b111),
               (String::from("MDA"), 0b111),
               (String::from("DAM"), 0b111),
               (String::from("ADM"), 0b111),
               (String::from("DMA"), 0b111),
               (String::from("MAD"), 0b111),
            ]),
            comp_table: HashMap::from([
               (String::from(""),    0b0101010),
               (String::from("0"),    0b0101010),
               (String::from("1"),   0b0111111),
               (String::from("-1"),  0b0111010),
               (String::from("D"),   0b0001100),
               (String::from("A"),   0b0110000),
               (String::from("!D"),  0b0001101),
               (String::from("!A"),  0b0110001),
               (String::from("-D"),  0b0001111),
               (String::from("-A"),  0b0110011),
               (String::from("D+1"), 0b0011111),
               (String::from("1+D"), 0b0011111),
               (String::from("A+1"), 0b0110111),
               (String::from("1+A"), 0b0110111),
               (String::from("D-1"), 0b0001110),
               (String::from("-1+D"),0b0001110),
               (String::from("A-1"), 0b0110010),
               (String::from("-1+A"),0b0110010),
               (String::from("D+A"), 0b0000010),
               (String::from("A+D"), 0b0000010),
               (String::from("D-A"), 0b0010011),
               (String::from("-A+D"),0b0010011),
               (String::from("A-D"), 0b0000111),
               (String::from("-D+A"),0b0000111),
               (String::from("D&A"), 0b0000000),
               (String::from("A&D"), 0b0000000),
               (String::from("D|A"), 0b0010101),
               (String::from("A|D"), 0b0010101),
               (String::from("M"),   0b1110000),
               (String::from("!M"),  0b1110001),
               (String::from("-M"),  0b1110011),
               (String::from("M+1"), 0b1110111),
               (String::from("1+M"), 0b1110111),
               (String::from("M-1"), 0b1110010),
               (String::from("-1+M"),0b1110010),
               (String::from("D+M"), 0b1000010),
               (String::from("M+D"), 0b1000010),
               (String::from("D-M"), 0b1010011),
               (String::from("-M+D"),0b1010011),
               (String::from("M-D"), 0b1000111),
               (String::from("-D+M"),0b1000111),
               (String::from("D&M"), 0b1000000),
               (String::from("M&D"), 0b1000000),
               (String::from("D|M"), 0b1010101),
               (String::from("M|D"), 0b1010101),
        
            ])
        };
        tables
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand
}

fn main() {
    // Initialize tables and bring in arguments
    let mut tables = Tables::new();

    let args: Vec<String> = env::args().collect();

    // Open .asm file and create a .hack file with the same name
    let mut file_name = args[1].clone();

    let mut file_in = {
        fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&file_name)
        .expect(&format!("Could not open {}!", &file_name))
    };

    file_name = file_name.replace(".asm", ".hack");

    // Create a reading buffer for input file
    let reader = BufReader::new(&file_in);
    
    // Fill in symbol table with the L commands from the file
    build_table(reader, &mut tables.symbol_table);
    file_in.rewind().unwrap();

    // Parse through the file and insert the commands into the command vector
    let mut commands: Vec<Command> = Vec::new();
    let new_reader = BufReader::new(&file_in);
    parse(new_reader, &mut commands, &mut tables);

    // Close the input file
    drop(file_in);

    // Create a string containing the binary representations of each command
    let write_string = build_binary(&commands);

    // Write to file 
    fs::write(&file_name, write_string).expect("Could not write to file");

}

fn build_table<'a>(reader: BufReader<&fs::File>, symbol_table: &'a mut  HashMap<String, i32>) {
    
    // Read input file line by line
    let mut pc = 0;
    for line in reader.lines() {
        let mut line = line.unwrap();

        // If the line is empty, move on to the next line
        line = remove_comment(line);
        if line.is_empty() {continue;}

        // If an L command exists on the line, add it to the symbol table. Otherwise,
        // increment the program counter
        if line.contains("(") {
            let first_split: Vec<&str> = line.split("(").collect();
            let second_split: Vec<&str> = first_split[1].split(")").collect();
            symbol_table.insert(second_split[0].to_string(), pc);
        } else {
            pc += 1;
        }
    }
}


fn parse<'a>(reader: BufReader<&fs::File>, commands: &mut Vec<Command>, tables: &'a mut  Tables) {
    // Read input file line by line
    let mut var_address = 16;
    for line in reader.lines() {
        let mut line = line.unwrap();

        // Remove comments from the lines
        line = remove_comment(line);

        // If line contains spaces, remove them
        if line.contains(" ") {
            let mut line_split: Vec<&str> = line.split(" ").collect();
            line_split.retain(|value| *value != "");

            line = line_split.join("");
        }
        if line.is_empty() {continue;}

        // Insert a new command into the vector initialized with the current line
        let mut currcommand = Command::new(&line);

        if currcommand.c_type == CommandType::LCommand {continue;}
        else if currcommand.c_type == CommandType::ACommand {

            // Check to see if A command contains variable
            let variable: bool = {
                let mut return_val: bool = false;
                for digit in currcommand.symbol.chars() {
                    if !digit.is_numeric() {
                        return_val = true;
                    }
                }
                return_val
            };
            
            // If a variable is found, check if it exists in the symbol table. 
            // If it doesn't, put it in the symbol table
            if variable {
                if !tables.symbol_table.contains_key(&currcommand.symbol[..]) {
                    tables.symbol_table.insert(String::from(&currcommand.symbol), var_address);
                    var_address += 1;
                }

                currcommand.symbol = tables.symbol_table
                    .get(&currcommand.symbol[..])
                    .unwrap()
                    .to_string();
            }

        } else if currcommand.c_type == CommandType::CCommand {
            // Fetch values for the destination, jump, and compare
            currcommand.dest = tables.lookup_table
            .get(&currcommand.dest[..])
            .unwrap()
            .to_string();

            currcommand.jump = tables.lookup_table
            .get(&currcommand.jump[..])
            .unwrap()
            .to_string();

            currcommand.comp = tables.comp_table
            .get(&currcommand.comp[..])
            .unwrap()
            .to_string();
        }
        commands.push(currcommand);
    }
}


fn build_binary (commands: &Vec<Command>) -> String {
    let mut output = String::new();
    for command in commands {
        
        if command.c_type == CommandType::LCommand {continue;}

        // Set first part of output string depending on command type
        let mut binary = String::new();
        
        if command.c_type == CommandType::CCommand {
            binary = 111.to_string() + 
            &format!("{:07b}", &command.comp
                .parse::<u16>()
                .unwrap())
            [..] + 
            &format!("{:03b}", &command.dest
                .parse::<u16>()
                .unwrap())
            [..] + 
            &format!("{:03b}", &command.jump
                .parse::<u16>()
                .unwrap())
            [..];
        }
        else {
            binary = 0.to_string() + 
            &format!("{:015b}", &command.symbol
                .parse::<u16>()
                .unwrap())
            [..];
        }

        output = output + &binary;
        output.push('\n');
    }
    output
}

fn remove_comment(line: String) -> String {
    let mut output = String::from(&line);
    if line.contains("//") {
        let slashes: Vec<&str> = line.split("//").collect();
        output = slashes[0].to_string();
    }
    output
}




