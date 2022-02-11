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

    pub fn run_instruction(&mut self, memory: &mut Ram){

        let lo = memory.read_byte(self.program_counter) as u16;
        let hi = memory.read_byte(self.program_counter+1) as u16;
        let instruction = (lo << 8) | hi;
        println!("{:#X} , {:#X} , {:#X}", instruction, lo, hi);
        if hi == 0 && lo == 0 {
            panic!();
        }
        //reads 2 bytes at a time
        self.program_counter += 2;
    }

}