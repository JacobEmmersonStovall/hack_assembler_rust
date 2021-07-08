use std::env;
use std::fs;

mod symbol_table;
mod code_translate;
mod assembler;

fn get_filename() -> String {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("The wrong number of arguments were given");
    }
    String::from(&args[1])
}

fn main() {
    let filename = get_filename();
    let file_contents = fs::read_to_string(&filename)
        .expect("Something went wrong reading the file");
    let binary_code = assembler::assemble_from_string(&file_contents).join("\n").to_string();
    let write_filename = filename[0..filename.len() - 4].to_string() + &".hack".to_string();
    fs::write(write_filename, binary_code).expect("Can't write to file");
}
