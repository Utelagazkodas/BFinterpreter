use std::{*};
use std::collections::HashMap;
mod bfcommands;

fn main() {


    let mut commands : HashMap<char, fn()> = HashMap::new();

    commands = initializeHashmap(commands);

    let mut source = fs::read_to_string("bf/main.bf")
        .expect("cum");

    source = removeWhiteSpace(source, &commands);

    for command in source.chars(){
        if let Some(&function) = commands.get(&command) {
            function();
        }
    }
}

fn removeWhiteSpace(t : String, commands : &HashMap<char, fn()>) -> String{
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

    //sub and add
    commands.insert("+".chars().next().unwrap(), || {bfcommands::add();});

    commands.insert("-".chars().next().unwrap(), || {bfcommands::subtract();});

    //movement
    commands.insert(">".chars().next().unwrap(), || {bfcommands::moveUp();});

    commands.insert("<".chars().next().unwrap(), || {bfcommands::moveDown();});


    //in and out put
    commands.insert(",".chars().next().unwrap(), || {bfcommands::input();});

    commands.insert(".".chars().next().unwrap(), || {bfcommands::output();});

    //loop
    commands.insert("[".chars().next().unwrap(), || {bfcommands::loopStart();});

    commands.insert("]".chars().next().unwrap(), || {bfcommands::LoopEnd();});

    return commands;
}