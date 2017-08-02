use keys;
use display;

pub struct Cpu<'a> {
    pc: u16,
    sp: u8,
    v: [u8; 16],
    i: u16,
    stack: [u16; 16],
    ram: [u8; 4096],
    display: &'a mut display::Display,
    keys: &'a mut keys::Keys,
}

impl<'a> Cpu<'a> {
    pub fn new(keys: &'a mut keys::Keys, display: &'a mut display::Display) -> Self{
        Self{
            pc: 0,
            sp: 0,
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            ram: [0; 4096],
            display: display,
            keys: keys,
        }
    }

    //wrapper to solve lifetime issue
    pub fn poll_keys(&mut self) -> Option<[bool; 16]> {
        self.keys.poll_keys()
    }
}
    