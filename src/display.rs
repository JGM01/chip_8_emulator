const WIDTH: usize = 64;
const HEIGHT: usize = 64;

pub struct Display {
    screen: [[u8; WIDTH]; HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[0; WIDTH]; HEIGHT],
        }
    }

    pub fn debug_draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool{
        let mut flipped = false;
        for _ in 0..8 {
            let mut b = byte;
            let x_coord = x as usize;
            let y_coord = y as usize;
            match (b & 0b1000_0000) >> 7 {
                0 => {
                    if self.screen[x_coord][y_coord] == 1 {
                        flipped = true;
                    }
                    self.screen[x_coord][y_coord] = 0;
                },
                1 => self.screen[x_coord][y_coord] = 1,
                _ => unreachable!(),
            }
            b = b << 1;
        }
        self.present();
        flipped
    }

    fn present(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.screen[x][y] == 0 {
                    print!("_");
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
    }
    
}
