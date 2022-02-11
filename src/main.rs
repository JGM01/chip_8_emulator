use std::{fs::File, io::Read};

use cpu::CPU;

mod cpu;
mod ram;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = CPU::new();
    chip8.load_rom(&data);
}
