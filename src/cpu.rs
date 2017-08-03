use keys;
use display;
use std::time::{Instant, Duration};

pub struct Cpu<'a> {
    sound_timestamp: Instant,
    delay_timestamp: Instant,
    //program counter
    pc: usize,
    //stack pointer
    sp: usize,
    //timers
    dt: u16,
    st: u16,
    //gp registers
    v: [u8; 16],
    i: u16,
    stack: [u16; 16],
    ram: [u8; 4096],
    display: &'a mut display::Display,
    keys: &'a mut keys::Keys,
}

impl<'a> Cpu<'a> {
    pub fn new(keys: &'a mut keys::Keys,
               display: &'a mut display::Display,
               program: Vec<u8>) 
               -> Self{

        Self{
            sound_timestamp: Instant::now(),
            delay_timestamp: Instant::now(),
            pc: 0x200,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            ram: {
                let offset = 0x200;
               
                let mut vector = Cpu::create_interpreter_space();
                vector.extend(program.iter().cloned());

                let mut ram = Cpu::from_slice(&vector);
                //debug:
                println!("{:?}", &ram.to_vec());
                //assert_eq!(ram.to_vec().len(), 4096 as usize);
                ram
            }
            ,
            display: display,
            keys: keys,
        }
    }

    //creates an array from vector slice
    fn from_slice(bytes: &[u8]) -> [u8; 4096] {
        let mut a = [0; 4096];
        for i in 0..a.len() {
            match bytes.get(i) {
                Some(e) => a[i] = e.clone(),
                None => a[i] = 0
            }
        }
        a
    }

    //allocates first 512 bytes and fills
    //them with font sprites
    pub fn create_interpreter_space() -> Vec<u8>{
        
        let mut font: Vec<u8> = vec![
            0xF0,0x90,0x90,0x90,0xF0, //0
            0x20,0x60,0x20,0x20,0x70, //1
            0xF0,0x10,0xF0,0x80,0xF0, //2
            0xF0,0x10,0xF0,0x10,0xF0, //3
            0x90,0x90,0xF0,0x10,0x10, //4
            0xF0,0x80,0xF0,0x10,0xF0, //5
            0xF0,0x80,0xF0,0x90,0xF0, //6
            0xF0,0x10,0x20,0x40,0x40, //7
            0xF0,0x90,0xF0,0x90,0xF0, //8
            0xF0,0x90,0xF0,0x10,0xF0, //9
            0xF0,0x90,0xF0,0x90,0x90, //A
            0xE0,0x90,0xE0,0x90,0xE0, //B
            0xF0,0x80,0x80,0x80,0xF0, //C
            0xE0,0x90,0x90,0x90,0xE0, //D
            0xF0,0x80,0xF0,0x80,0xF0, //E
            0xF0,0x80,0xF0,0x80,0x80  //F
            ];
        let vector: Vec<u8> = vec![0; 0x200-font.len()];
        font.extend(vector);
        assert_eq!(font.len(), 0x200 as usize);
        font
    }

    //cpu clock tick
    pub fn tick(&mut self) {
        if self.dt > 0 
            && self.delay_timestamp.elapsed() > Duration::from_millis(17) {
                self.dt -= 1;
                self.delay_timestamp = Instant::now();
        }
        if self.st > 0 
            && self.sound_timestamp.elapsed() > Duration::from_millis(17) {
                self.st -= 1;
                self.sound_timestamp = Instant::now();
                println!("buzz")
        }
        self.execute_instruction();
        self.render();
    }

    pub fn execute_instruction(&mut self) {
        //construct an 16-bit opcode from 2 consec. bytes
        let opcode = (self.ram[self.pc as usize] as u16) << 8
            | (self.ram[self.pc as usize + 1] as u16);

        match opcode >> 11 {
            0x0 => self.op_0(opcode),
            0x1 => self.op_1(opcode),
            0x2 => self.op_2(opcode),
            0x3 => self.op_3(opcode),
            0x4 => self.op_4(opcode),
            0x5 => self.op_5(opcode),
            0x6 => self.op_6(opcode),
            0x7 => self.op_7(opcode),
            0x8 => self.op_8(opcode),
            0x9 => self.op_9(opcode),
            0xA => self.op_A(opcode),
            0xB => self.op_B(opcode),
            0xC => self.op_C(opcode),
            0xD => self.op_D(opcode),
            0xE => self.op_E(opcode),
            0xF => self.op_F(opcode),
            _ => {}
        }
    }

    //helper methods:
    //TODO:: check if assumptions true
    #[inline(always)]
    fn extract_nnn(opcode: &u16) -> u16{
        opcode & 0x0FFF
    }

    #[inline(always)]
    fn extract_kk(opcode: &u16) -> u8{
        (opcode & 0x00FF) as u8
    }

    #[inline(always)]
    fn extract_x(opcode: &u16) -> u8{
        ((opcode & 0x0F00) >> 8) as u8
    }

    #[inline(always)]
    fn extract_y(opcode: &u16) -> u8{
        ((opcode & 0x00F0) >> 4) as u8
    }

    #[inline(always)]
    fn extract_n(opcode: &u16) -> u8{
        (opcode & 0x000F) as u8
    }

    //instructions:

    //CLS / RET
    fn op_0(&mut self, opcode: u16) {
        match opcode {
            0x00E0 => self.display.clear(),
            0x00EE => self.pc = self.stack[self.sp] as usize,
            _ => panic!("Unknown opcode")
        }
        self.pc += 1;
    }

    //JP nnn
    fn op_1(&mut self, opcode: u16) {
        self.pc = Cpu::extract_nnn(&opcode) as usize;   
    }

    //CALL nnn
    fn op_2(&mut self, opcode: u16) {
        self.sp += 1;
        self.stack[self.sp] = self.pc as u16;
        self.pc = Cpu::extract_nnn(&opcode) as usize;
    }

    //SE Vx, kk
    fn op_3(&mut self, opcode: u16) {
        if self.v[Cpu::extract_x(&opcode) as usize] 
            == Cpu::extract_kk(&opcode) {
            self.pc += 2;
        }
        else {
            self.pc += 1;
        }
    }

    //SNE Vx, kk
    fn op_4(&mut self, opcode: u16) {
        if self.v[Cpu::extract_x(&opcode) as usize] 
            != Cpu::extract_kk(&opcode) {
            self.pc += 2;
        }
        else {
            self.pc += 1;
        }
    }

    //SE Vx, Vy
    fn op_5(&mut self, opcode: u16) {
        if self.v[Cpu::extract_x(&opcode) as usize] 
            == self.v[Cpu::extract_y(&opcode) as usize] {
            self.pc += 2;
        }
        else {
            self.pc += 1;
        }
    }

    //LD Vx, kk
    fn op_6(&mut self, opcode: u16) {
        self.v[Cpu::extract_x(&opcode) as usize]
            = Cpu::extract_kk(&opcode);
        self.pc += 1;
    }

    //ADD Vx, kk
    fn op_7(&mut self, opcode: u16) {
        self.v[Cpu::extract_x(&opcode) as usize]
            += Cpu::extract_kk(&opcode);
        self.pc += 1;
    }

    //LD Vx, Vy
    fn op_8(&mut self, opcode: u16) {
        match Cpu::extract_n(&opcode) {
            0x0 => {
                self.v[Cpu::extract_x(&opcode) as usize]
                    = self.v[Cpu::extract_y(&opcode) as usize];
                self.pc += 1;
            }
            _ => panic!("Unknown opcode")
        }
    }
    fn op_9(&mut self, opcode: u16) {}
    fn op_A(&mut self, opcode: u16) {}
    fn op_B(&mut self, opcode: u16) {}
    fn op_C(&mut self, opcode: u16) {}
    fn op_D(&mut self, opcode: u16) {}
    fn op_E(&mut self, opcode: u16) {}
    fn op_F(&mut self, opcode: u16) {}

    //wrappers to solve lifetime issues
    pub fn poll_keys(&mut self) -> Option<[bool; 16]> {
        self.keys.poll_keys()
    }

    pub fn render(&mut self) {
        self.display.render();
    }
}
    