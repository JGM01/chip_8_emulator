use crate::ram::Ram;

pub struct CPU {
    ram: Ram,
}

impl CPU {
    pub fn new() -> CPU {
        CPU { ram: Ram::new() }
    }

    pub fn load_rom(&mut self, ROM: &Vec<u8>) {
        let offset = 0x200;
        for (address, op_code) in ROM.iter().enumerate() {
            self.ram.write_byte(offset + address as u16, *op_code);
        }
    }
}
