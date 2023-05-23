use ::ascii::{AsAsciiStr, ToAsciiChar};
use lazy_static::lazy_static;
use core::panic;
use std::sync::Mutex;
use std::ascii;

pub static mut pointer : u8 = 0;

const ALL: [str; 128] = [
            "Null", "SOH", "SOX", "ETX", "EOT", "ENQ", "ACK", "Bell",
            "BackSpace", "Tab", "LineFeed", "VT", "FF", "CarriageReturn", "SI", "SO",
            "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB",
            CAN, EM, SUB, ESC, FS, GS, RS, US,
            Space, Exclamation, Quotation, Hash, Dollar, Percent, Ampersand, Apostrophe,
            ParenOpen, ParenClose, Asterisk, Plus, Comma, Minus, Dot, Slash,
            _0, _1, _2, _3, _4, _5, _6, _7,
            _8, _9, Colon, Semicolon, LessThan, Equal, GreaterThan, Question,
            At, A, B, C, D, E, F, G,
            H, I, J, K, L, M, N, O,
            P, Q, R, S, T, U, V, W,
            X, Y, Z, BracketOpen, BackSlash, BracketClose, Caret, UnderScore,
            Grave, a, b, c, d, e, f, g,
            h, i, j, k, l, m, n, o,
            "p", "q", "r", "s", "t", "u", "v", "w",
            "x", "y", "z", "CurlyBraceOpen", "VerticalBar", "CurlyBraceClose", "Tilde", "DEL",
		];

lazy_static! {
    pub static ref registers: Mutex<[u8; std::u8::MAX as usize]> = Mutex::new([0; std::u8::MAX as usize]);
}

// on +
pub fn add(){

    unsafe{
        let registers_mutex = &*registers;
        let mut registers_guard = registers_mutex.lock().unwrap();
        let pointer_usize = pointer as usize;

        if (*registers_guard)[pointer_usize] == u8::MAX {
            (*registers_guard)[pointer_usize] = 0;
            return;
        }

        (*registers_guard)[pointer_usize] += 1;
    }
}

// on -
pub fn subtract(){
    unsafe{


        let registers_mutex = &*registers;
        let mut registers_guard = registers_mutex.lock().unwrap();
        let pointer_usize = pointer as usize;

        if (*registers_guard)[pointer_usize] == 0 {
            (*registers_guard)[pointer_usize] = u8::MAX;
            return;
        }

        (*registers_guard)[pointer_usize] -= 1;
    }
}

// on >
pub fn moveUp(){

    unsafe{
        if pointer == u8::MAX {
            pointer = 0;
            return;
        }
        pointer += 1;
    }
    
}

// on <
pub fn moveDown(){

    unsafe{
        if pointer == 0 {
            pointer = u8::MAX;
            return;
        }
        pointer -= 1;
    }
}

// on ,
pub fn input(){
    unsafe{
        let registers_mutex = &*registers;
        let mut registers_guard = registers_mutex.lock().unwrap();
        let pointer_usize = pointer as usize; 

        let print = match (*registers_guard)[pointer_usize].to_ascii_char() {
            Ok(ty) => ty,
            Err(er) => panic!("AAAAAA {:?}", er),
        };

        print!("{}", print);
    }
}   

// on .
pub fn output(){ 
    unsafe{
        let registers_mutex = &*registers;
            let mut registers_guard = registers_mutex.lock().unwrap();
            let pointer_usize = pointer as usize; 
        print!("{}", (*registers_guard)[pointer_usize])
    }
}

// on [
pub fn loopStart(){
    print!("start")
}

// on ]
pub fn LoopEnd(){
    print!("end")
}

