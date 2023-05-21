use std::process::Command;
use std::ptr::null;
use std::{*, str::Chars, collections::btree_map::Range};
use std::collections::HashMap;
mod bfcommands;

static mut pointer : u16 = 0;


fn main() {
    let mut commands : HashMap<char, fn()> = HashMap::new();

    commands = initializeHashmap(commands);

    let mut source = fs::read_to_string("bf/main.bf")
        .expect("cum");

    source = removeWhiteSpace(source, commands);

    print!("{}", source)
}

fn removeWhiteSpace(t : String, commands : HashMap<char, fn()>) -> String{
    let mut ret = String::from("");

    for inst in t.chars(){
        if !inst.is_whitespace() && commands.contains_key(&inst){
            ret.push(inst)
        }
    }

    return ret;
}


// initializes the hashmap
fn initializeHashmap(mut commands : HashMap<char, fn()>) -> HashMap<char, fn()>{

    commands.insert("+".chars().next().unwrap(), || {bfcommands::add();});
    return commands;
}

// initializes the registers
fn initializeRegisters(){
    let registers: [u16; std::u16::MAX as usize] = [0; std::u16::MAX as usize];
}