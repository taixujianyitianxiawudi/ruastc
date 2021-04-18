// use crate::compiler::*;
use std::collections::HashMap;

pub fn generate_native_code(mid_commands: &Vec<String>) -> Vec<String> {
    let mut native_commands = Vec::new();

    native_commands.push(String::from(".text"));
    native_commands.push(String::from(".global main"));
    native_commands.push(String::from("main:"));
    

    let mut variable_map = HashMap::new();
    variable_map.insert("sss", 1);
    
    for command in mid_commands.iter() {
        let v: Vec<&str> = command[..].split(' ').collect();
        match v[0] {
            "PUSH" => {
                native_commands.push(format!("\tadd t0, x0, {}", v[1]));
                native_commands.push(format!("\tsd t0, -8(sp)"));
                native_commands.push(format!("\taddi sp, sp, -8"));
            }
            "RETURN" => {
                native_commands.push(format!("\tld a0, 0(sp)"));
                native_commands.push(format!("\taddi sp, sp, 8"));
                native_commands.push(format!("\tret"));
            }
            _ => {
                panic!("2211");
            }
        }
    }
    native_commands
}
