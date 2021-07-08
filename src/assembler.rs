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

pub fn assemble_from_string(code_to_assemble: &String) -> Vec<String> {
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
    let mut clean_lines_without_symbols = Vec::new();

    //Take out labels (symbols) from the code vector and make a new code vector
    //without symbols. Additionally, add the labels pointing to a line in the program
    //to go to (starting from index 0).
    for clean_line in clean_code_lines {
        if clean_line.starts_with("(") {
            let new_symbol = &clean_line[1..clean_line.len() - 1];
            symbol_table.add_entry(&new_symbol.to_string(), line_count);
        } else {
            line_count += 1;
            clean_lines_without_symbols.push(String::from(clean_line));
        }
    }
    
    let mut memory_store_start = 16;
    let mut binary_translations = Vec::new();
    for exec_line in clean_lines_without_symbols{
        //A-Instruction
        if exec_line.starts_with("@") {
            let address_to_look_at = exec_line[1..].to_string();
            let is_digit = address_to_look_at.chars().nth(0).unwrap().is_digit(10);
            if is_digit {
                let digit = address_to_look_at.parse::<u16>().unwrap();
                binary_translations.push(format!("{:016b}", digit));
            } else {
                if !symbol_table.contains(&address_to_look_at) {
                    symbol_table.add_entry(&address_to_look_at.to_string(), memory_store_start);
                    memory_store_start += 1;
                }
                binary_translations.push(format!("{:016b}", symbol_table.get_address(&address_to_look_at)));
            }
        } else { //C-Insturction
            let equal_symbol_index = exec_line.find("=");
            let (dest, second_part) = match equal_symbol_index {
                Some(value) => (exec_line[..value].to_string(),exec_line[value+1..].to_string()),
                None => ("".to_string(), exec_line)
            };

            let semicolon_index = second_part.find(";");
            let (comp, jump) = match semicolon_index {
                Some(value) => (second_part[..value].to_string(),second_part[value+1..].to_string()),
                None => (second_part, "".to_string())
            };

            let translated_c_instruction = "111".to_string() + &translate_comp(&comp).to_string() + &translate_dest(&dest).to_string() + &translate_jump(&jump).to_string();
            binary_translations.push(String::from(translated_c_instruction));
        }
    }
    binary_translations
}