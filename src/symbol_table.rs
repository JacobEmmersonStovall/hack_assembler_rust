use std::collections;

pub struct SymbolTable {
    table: collections::HashMap<String, i32>
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut return_value = Self {
            table: collections::HashMap::new(),
        };
        //Reserved values
        return_value.table.insert("SP".to_string(), 0x0000);
        return_value.table.insert("LCL".to_string(), 0x0001);
        return_value.table.insert("ARG".to_string(), 0x0002);
        return_value.table.insert("THIS".to_string(), 0x0003);
        return_value.table.insert("THAT".to_string(), 0x0004);

        return_value.table.insert("R0".to_string(), 0x0000);
        return_value.table.insert("R1".to_string(), 0x0001);
        return_value.table.insert("R2".to_string(), 0x0002);
        return_value.table.insert("R3".to_string(), 0x0003);
        return_value.table.insert("R4".to_string(), 0x0004);
        return_value.table.insert("R5".to_string(), 0x0005);
        return_value.table.insert("R6".to_string(), 0x0006);
        return_value.table.insert("R7".to_string(), 0x0007);
        return_value.table.insert("R8".to_string(), 0x0008);
        return_value.table.insert("R9".to_string(), 0x0009);
        return_value.table.insert("R10".to_string(), 0x000A);
        return_value.table.insert("R11".to_string(), 0x000B);
        return_value.table.insert("R12".to_string(), 0x000C);
        return_value.table.insert("R13".to_string(), 0x000D);
        return_value.table.insert("R14".to_string(), 0x000E);
        return_value.table.insert("R15".to_string(), 0x000F);

        return_value.table.insert("SCREEN".to_string(), 0x4000);
        return_value.table.insert("KBD".to_string(), 0x6000);
        
        //Return structure
        return_value
    }

    pub fn contains(&self, symbol: &String) -> bool{
        self.get_address(symbol) >= 0
    }

    pub fn get_address(&self, symbol: &String) -> i32 {
        match self.table.get(symbol) {
            None => return -1,
            Some(value) => return *value
        }
    }

    pub fn add_entry(&mut self, symbol: &String, address: i32) -> (bool, i32) {
        let get_address_value = self.get_address(symbol);
        if get_address_value >= 0 {
            return (false, get_address_value);
        }
        self.table.insert(symbol.to_string(), address);
        (true, address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test(){
        let mut test_table = SymbolTable::new();
        assert_eq!(test_table.contains(&"SP".to_string()), true);
        assert_eq!(test_table.get_address(&"NonExistent".to_string()), -1);
        assert_eq!(test_table.add_entry(&"SP".to_string(), 32), (false, 0));
    }

    #[test]
    fn add_entry_test(){
        let mut test_table = SymbolTable::new();
        assert_eq!(test_table.contains(&"KEY".to_string()), false);
        assert_eq!(test_table.add_entry(&"KEY".to_string(), 0x428), (true, 0x428));
        assert_eq!(test_table.get_address(&"KEY".to_string()), 0x428);
        
    }
}