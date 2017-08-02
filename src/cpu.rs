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
    pub fn new(keys: &'a mut keys::Keys,
               display: &'a mut display::Display,
               program: Vec<u8>) 
               -> Self{

        Self{
            pc: 0x200,
            sp: 0,
            v: [0; 16],
            i: 0,
            stack: [0; 16],
            ram: {
                let offset = 0x200;
               
                let mut vector = Cpu::create_interpreter_space();
                vector.extend(program.iter().cloned());

                let mut ram = Cpu::from_slice(&vector);
                //debug:
                println!("{:?}", &vector);
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
    //them with preset sprites
    pub fn create_interpreter_space() -> Vec<u8>{
        let mut vector: Vec<u8> = vec![0; 0x200];
        vector
        //TODO
    }

    //wrapper to solve lifetime issue
    pub fn poll_keys(&mut self) -> Option<[bool; 16]> {
        self.keys.poll_keys()
    }
}
    