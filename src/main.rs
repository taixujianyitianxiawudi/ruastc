use std::env;
use std::fs;

pub mod compiler;
fn main() {
    let filename: Vec<String> = env::args().collect();
    let dir = "tests/step1/";
    let filedir = dir.to_owned()+&filename[1];
    
    let contents: String = fs::read_to_string(filedir)
        .expect("Something went wrong reading the file");
    println!("{}", contents);

    let tokens = compiler::lexing(&contents);
    for s in tokens.iter() {
        println!("{:?}-----------{:?}",s.kind, s.val);
    }

    let ast = compiler::parsing(&tokens);

    let mid_commands = compiler::generate_intermediate_code(&ast);
    for command in mid_commands.iter() {
        println!("{}", command);
    }
    
    let native_commands = compiler::generate_native_code(&mid_commands);

    for command in native_commands.iter() {
        println!("{}", command);
    }
}
