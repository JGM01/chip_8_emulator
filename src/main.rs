extern crate minifb;
extern crate rand;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use display::Display;
use std::time::{Duration, Instant};
use std::env;

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



    let WIDTH = 640;
    let HEIGHT = 320;

    let mut buffer : Vec<u32> = vec![0; WIDTH*HEIGHT];
    for i in buffer.iter_mut() {
        *i = 0xFFF;
    }
    
    let mut window = Window::new("Chip-8", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {panic!("{}", e);});

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    
    while(window.is_open()) {
        
        chip8.run_instruction();
        let chip8_buffer = chip8.get_display_buffer();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = Display::get_index_from_coords(x/10 , y/10);
                let pixel = chip8_buffer[index];
                let color_pixel = match pixel {
                    0 => 0x0,
                    1 => 0xFFFFFF,
                    _ => unreachable!(),
                };
                buffer[y * WIDTH + x] = color_pixel;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

}
