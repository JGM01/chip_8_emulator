use crate::ram::Ram;
use crate::cpu::CPU;
use crate::cpu;

pub struct Chip8 {
    ram: Ram,
    cpu: CPU,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { 
            ram: Ram::new(), 
            cpu: CPU::new(),
        }
    }

    pub fn load_rom(&mut self, ROM: &Vec<u8>) {
        for (address, op_code) in ROM.iter().enumerate() {
            self.ram.write_byte(cpu::PROGRAM_START + (address as u16), *op_code);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.ram);
        println!("CPU State: {:?}", self.cpu)
    }

}
