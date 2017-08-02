struct Cpu {
    pc: u16,
    sp: u8,
    v: [u8; 16],
    i: u16,
    stack: [u16; 16],
    ram: [u8; 4096],
}

impl Cpu {
    
}