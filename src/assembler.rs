use crate::code_translate::translate_dest;
use crate::code_translate::translate_jump;
use crate::code_translate::translate_comp;
use crate::symbol_table::SymbolTable;

fn clean_up_line(line: &String) -> String {
    let without_whitespace = line.split_whitespace().collect::<Vec<&str>>();
    let without_whitsepace_string = without_whitespace.join("").to_string();
    let index_of_comment = without_whitsepace_string.find("//").unwrap_or(usize::MAX);
    let mut return_string = without_whitsepace_string;
    if index_of_comment != usize::MAX {
        let return_str = &return_string[..index_of_comment];
        return_string = return_str.to_string();
    }
    return_string
}

pub fn assemble_from_string(code_to_assemble: &String) {
    let lines = code_to_assemble.split("\n").collect::<Vec<&str>>();
    for line in lines {
        let cleaned_up_line = &clean_up_line(&line.to_string());
        if cleaned_up_line != "" {
            println!("{}", cleaned_up_line);
        }
    }
}