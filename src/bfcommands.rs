use lazy_static::lazy_static;
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
        (*registers_guard)[pointer_usize] += 1;
    }
}

// on -
pub fn subtract(){
    unsafe{
        let registers_mutex = &*registers;
        let mut registers_guard = registers_mutex.lock().unwrap();
        let pointer_usize = pointer as usize;
        (*registers_guard)[pointer_usize] -= 1;
    }
}

// on >
pub fn moveUp(){

    unsafe{
        pointer += 1;
    }
    
}

// on <
pub fn moveDown(){

    unsafe{
        pointer -= 1;
    }
}

// on ,
pub fn input(){
    print!("inp")
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

