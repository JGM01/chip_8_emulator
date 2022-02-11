use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

pub struct CPU {
    vx_register: [u8;16],
    i_register: u16,
    program_counter: u16,

}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            vx_register: [0; 16],
            i_register: 0,
            program_counter: PROGRAM_START,
        }
    }

    pub fn run_instruction(&mut self, capacity: &mut Ram){

    }

}