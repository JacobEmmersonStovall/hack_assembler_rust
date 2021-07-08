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
    //Split the code into individual lines
    let lines = code_to_assemble.split("\n").collect::<Vec<&str>>();
    //Create a vector to store executable lines
    let mut clean_code_lines = Vec::new();
    //Clean each line and if there is executable code, push it into the vector
    for line in lines {
        let cleaned_up_line = &clean_up_line(&line.to_string());
        if cleaned_up_line != "" {
            clean_code_lines.push(String::from(cleaned_up_line));
        }
    }

    //Create a symbol table
    let mut symbol_table = SymbolTable::new();
    //Current line count
    let mut line_count = 0;

    println!("Cleaned Lines: {:?}", clean_code_lines);

    let mut clean_lines_without_symbols = Vec::new();

    for clean_line in clean_code_lines {
        if clean_line.starts_with("(") {
            let new_symbol = &clean_line[1..clean_line.len() - 1];
            symbol_table.add_entry(&new_symbol.to_string(), line_count);
            //println!("{}", new_symbol);
        } else {
            line_count += 1;
            clean_lines_without_symbols.push(String::from(clean_line));
            //println!("{}", clean_line);
        }
    }

    println!("Lines without Symbols: {:?}", clean_lines_without_symbols);
    //println!("{}", symbol_table.get_address(&"After".to_string()));
}