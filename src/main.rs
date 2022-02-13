extern crate minifb;

use std::{fs::File, io::Read};

use chip8::Chip8;

mod chip8;
mod cpu;
mod display;
mod keyboard;
mod ram;
mod bus;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    loop {
        chip8.run_instruction();
    }
}
