use std::env;

fn get_filename() -> String {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("The wrong number of arguments were given");
    }
    String::from(&args[1])
}

fn main() {
    let filename = get_filename();
    println!("Filename: {}", filename);
}
