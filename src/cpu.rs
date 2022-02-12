use crate::bus::Bus;

pub const PROGRAM_START: u16 = 0x200;

pub struct CPU {
    vx_register: [u8; 16],
    i_register: u16,
    program_counter: u16,
    previous_program_counter: u16,
    return_stack: Vec<u16>,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            vx_register: [0; 16],
            i_register: 0,
            program_counter: PROGRAM_START,
            previous_program_counter: 0,
            return_stack: Vec::<u16>::new(),
        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let lo = bus.ram_read_byte(self.program_counter) as u16;
        let hi = bus.ram_read_byte(self.program_counter + 1) as u16;
        let instruction = (lo << 8) | hi;
        println!("{:#X} , {:#X} , {:#X}", instruction, lo, hi);

        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x00F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;

        println!("{:?} , {:?} , {:?} , {} , {}", nnn, nn, n, x, y);

        if self.previous_program_counter == self.program_counter {
            panic!("Increment!!!");
        }
        self.previous_program_counter = self.program_counter;

        match (instruction & 0xF000) >> 12 {
            0x0 => match nn {
                0xE0 => {
                    bus.clear_screen();
                    self.program_counter += 2;
                }
                0xEE => {
                    let subroutine_return = self.return_stack.pop().unwrap();
                    self.program_counter = subroutine_return;
                }
                _ => panic!(
                    "[0x0NN] Unrecognizable! {:#X} , {:#X}",
                    self.program_counter, instruction
                ),
            },
            0x1 => {
                self.program_counter = nnn;
            }
            0x2 => {
                self.return_stack.push(self.program_counter + 2);
                self.program_counter = nnn;
            }
            0x3 => {
                let vx = self.read_vx_register(x);
                if vx == nn {
                    self.program_counter += 4;
                } else {
                    self.program_counter += 2;
                }
            }
            0x6 => {
                self.write_vx_register(x, nn);
                self.program_counter += 2;
            }
            0x7 => {
                let vx = self.read_vx_register(x);
                self.write_vx_register(x, vx.wrapping_add(nn));
                self.program_counter += 2;
            }
            0x8 => {
                let vx = self.read_vx_register(x);
                let vy = self.read_vx_register(y);
                match n {
                    0 => {
                        self.write_vx_register(x, vy);
                    }
                    1 => {
                        self.write_vx_register(x, vx | vy);
                    }
                    2 => {
                        self.write_vx_register(x, vx & vy);
                    }
                    3 => {
                        self.write_vx_register(x, vx ^ vy);
                    }
                    _ => panic!(
                        "[0x8XYN] Unrecognizable! {:#X} , {:#X}",
                        self.program_counter, instruction
                    ),
                };
                self.program_counter += 2;
            }
            0xA => {
                self.i_register = nnn;
                self.program_counter += 2;
            }
            0xD => {
                //Draw frame (will to later lol)
                self.debug_draw_sprite(bus, x, y, n);
                self.program_counter += 2;
            }
            0xE => {
                match nn {
                    0xA1 => {
                        let key = self.read_vx_register(x);
                        if bus.key_press(key) {
                            self.program_counter += 2;
                        } else {
                            self.program_counter += 4;
                        }
                    }
                    0x9E => {}
                    _ => panic!(
                        "[0xEXNN] Unrecognizable! {:#X} , {:#X}",
                        self.program_counter, instruction
                    ),
                };
            }
            0xF => {
                let vx = self.read_vx_register(x);
                self.i_register += vx as u16;
                self.program_counter += 2;
            }
            _ => panic!(
                "[0xNNNN] Unrecognizable! {:#X} , {:#X}",
                self.program_counter, instruction
            ),
        }
    }

    fn debug_draw_sprite(&mut self, bus: &mut Bus, x: u8, y: u8, height: u8) {
        let mut should_set_vx = false;
        for y in 0..height {
            let b = bus.ram_read_byte(self.i_register + y as u16);
            if bus.debug_draw_byte(b, x, y) {
                should_set_vx = true;
            }
        }
        if should_set_vx {
            self.write_vx_register(0xF, 0);
        } else {
            self.write_vx_register(0xF, 0);
        }
        bus.present_screen();
    }

    pub fn write_vx_register(&mut self, address: u8, value: u8) {
        self.vx_register[address as usize] = value;
    }
    pub fn read_vx_register(&mut self, address: u8) -> u8 {
        self.vx_register[address as usize]
    }
}
