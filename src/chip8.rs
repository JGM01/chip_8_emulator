use crate::ram::Ram;

pub struct Chip8 {
    ram: Ram,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 { ram: Ram::new() }
    }

    pub fn load_rom(&mut self, ROM: &Vec<u8>) {
        let offset = 0x200;
        for (address, op_code) in ROM.iter().enumerate() {
            self.ram.write_byte(offset + address as u16, *op_code);
        }
    }

    pub fn run_instruction(&mut self) {
        
    }

}
