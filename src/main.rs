use std::{fs::File, io::Read};

mod cpu;
mod ram;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);    



}
