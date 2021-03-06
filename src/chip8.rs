use crate::cpu;
use crate::cpu::CPU;
use crate::bus::Bus;


pub struct Chip8 {
    bus: Bus,
    cpu: CPU,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            bus: Bus::new(),
            cpu: CPU::new(),
        }
    }

    pub fn load_rom(&mut self, ROM: &Vec<u8>) {
        for (address, op_code) in ROM.iter().enumerate() {
            self.bus
                .ram_write_byte(cpu::PROGRAM_START + (address as u16), *op_code);
        }
    }

    pub fn run_instruction(&mut self) {
        self.cpu.run_instruction(&mut self.bus);
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        self.bus.get_display_buffer()
    }

    pub fn set_key_pressed(&mut self, key: Option<u8>) {
        self.bus.set_key_pressed(key);
    }
}