use std::env;
use std::fs;
mod symbol_table;

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
    println!("Filename: {}", filename);
    println!("File Contents:\n{}", file_contents);
}
