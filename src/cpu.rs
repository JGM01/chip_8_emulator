use crate::ram::Ram;

struct CPU {
    ram: Ram,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            ram: Ram::new(),
        }
    }
}