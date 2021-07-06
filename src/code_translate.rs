pub fn translate_dest(dest_str: &str) -> &str {
    match dest_str {
        "" => "000",
        "M" => "001",
        "D" => "010",
        "MD" | "DM" => "011",
        "A" => "100",
        "AM" | "MA" => "101",
        "AD" | "DA" => "110",
        "AMD" | "ADM" | "MDA" | "MAD" | "DMA" | "DAM" => "111",
        _ => panic!("Unexpected Destination String")
    }
}

pub fn translate_jump(jump_str: &str) -> &str {
    match jump_str {
        "" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => panic!("Unexpected Jump String")
    }
}

pub fn translate_comp(comp_str: &str) -> &str {
    match comp_str {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "M" => "1110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "!M" => "1110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "-M" => "1110011",
        "D+1" | "1+D" => "0011111",
        "A+1" | "1+A" => "0110111",
        "M+1" | "1+M" => "1110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "M-1" => "1110010",
        "D+A" | "A+D" => "0000010",
        "D+M" | "M+D" => "1000010",
        "D-A" => "0010011",
        "D-M" => "1010011",
        "A-D" => "0000111",
        "M-D" => "1000111",
        "D&A" | "A&D" => "0000000",
        "M&D" | "D&M" => "1000000",
        "D|A" | "A|D" => "0010101",
        "D|M" | "M|D" => "1010101",
        _ => panic!("Unexpected Comp String")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_dest_test(){
        assert_eq!(translate_dest("M"), "001");
        assert_eq!(translate_dest("A"), "100");
        assert_eq!(translate_dest("D"), "010");
        assert_eq!(translate_dest("DA"), "110");
        assert_eq!(translate_dest("DM"), "011");
        assert_eq!(translate_dest("MA"), "101");
        assert_eq!(translate_dest("MAD"), "111");
        assert_eq!(translate_dest(""), "000");
    }

    #[test]
    fn translate_jump_test(){
        assert_eq!(translate_jump("JGT"), "001");
        assert_eq!(translate_jump("JLT"), "100");
        assert_eq!(translate_jump("JEQ"), "010");
        assert_eq!(translate_jump("JLE"), "110");
        assert_eq!(translate_jump("JGE"), "011");
        assert_eq!(translate_jump("JNE"), "101");
        assert_eq!(translate_jump("JMP"), "111");
        assert_eq!(translate_jump(""), "000");
    }

    #[test]
    fn translate_comp_test(){
        assert_eq!(translate_comp("-D"), "0001111");
        assert_eq!(translate_comp("A+D"), "0000010");
        assert_eq!(translate_comp("M-1"), "1110010");
    }
}