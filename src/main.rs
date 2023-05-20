use std::{*, str::Chars, collections::btree_map::Range};

static mut pointer : u16 = 0;

fn main() {
    let mut source = fs::read_to_string("bf/main.bf")
    .expect("cum");

    source = removeWhiteSpace(source);


}

fn removeWhiteSpace(t : String) -> String{
    let mut ret = String::from("");

    for inst in t.chars(){
        if !inst.is_whitespace() {
            ret.push(inst)
        }
    }

    return ret;
}

