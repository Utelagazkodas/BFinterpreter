use std::fmt::Write;
use std::ptr::null;
use std::{*};
use std::collections::HashMap;
mod bfcommands;

static mut source : Option<String> = None;

fn main() {


    let mut commands : HashMap<char, fn()> = HashMap::new();

    commands = initializeHashmap(commands);

    unsafe {
        //reads and formats the file
        source = Some(removeWhiteSpace(fs::read_to_string("bf/main.bf").unwrap(), &commands));
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
    commands.insert("[".chars().next().unwrap(), || {loopStart();});

    commands.insert("]".chars().next().unwrap(), || {LoopEnd();});

    return commands;
}



// on [
pub fn loopStart(){
    print!("start")
}

// on ]
pub fn LoopEnd(){
    print!("end")
}

pub fn runBFfrom(from : u128){
    let file = "".to_string();

    unsafe{
        file = source.clone().unwrap();
    }


    for command in source.unwrap().chars(){
        if let Some(&function) = commands.get(&command) {
            function();
        }
    }
}