
use lazy_static::lazy_static;
use core::panic;
use std::sync::Mutex;
use std::ascii;

pub static mut pointer : u8 = 0;

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
    // TO BE IMPLEMENTED
}   

// on .
pub fn output(){ 
    unsafe{
        let registers_mutex = &*registers;
            let mut registers_guard = registers_mutex.lock().unwrap();
            let pointer_usize = pointer as usize; 
        print!("{}", std::char::from_u32((*registers_guard)[pointer_usize] as u32).unwrap())
    }
}

