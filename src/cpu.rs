use core::panic;

use crate::ram::Ram;

pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
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

        let nnn = instruction & 0x0FFF;
        let nn = instruction & 0x0FF;
        let n = instruction & 0x00F;
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;

        println!("{:?} , {:?} , {:?} , {} , {}", nnn, nn, n, x, y);

        match (instruction & 0xF000) >> 12 {
            0x1 => {
                self.program_counter = nnn;
            }
            _ => panic!("Unrecognizable! {:#X} , {:#X}", self.program_counter, instruction)
        }
    }
}