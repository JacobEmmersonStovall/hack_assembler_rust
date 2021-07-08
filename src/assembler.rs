use crate::code_translate::translate_dest;
use crate::code_translate::translate_jump;
use crate::code_translate::translate_comp;
use crate::symbol_table::SymbolTable;

fn clean_up_line(line: &String) -> String {
    let without_whitespace = line.split_whitespace().collect::<Vec<&str>>();
    let without_whitsepace_string = without_whitespace.join("").to_string();

    let index_of_comment = without_whitsepace_string.find("//");
    match index_of_comment {
        Some(value) => String::from(&without_whitsepace_string[..value]),
        None => without_whitsepace_string,
    }
}

pub fn assemble_from_string(code_to_assemble: &String) {
    let lines = code_to_assemble.split("\n").collect::<Vec<&str>>();
    let mut clean_code_lines = Vec::new();
    for line in lines {
        let cleaned_up_line = &clean_up_line(&line.to_string());
        if cleaned_up_line != "" {
            clean_code_lines.push(String::from(cleaned_up_line));
        }
    }

    println!("{:?}", clean_code_lines);
}