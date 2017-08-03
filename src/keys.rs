

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct SDLKeys {
    event_pump: sdl2::EventPump,
    keys: [bool; 16]
}

pub trait Keys {
    fn poll_keys(&mut self) -> Option<[bool; 16]>;
    fn get_keys(&self) -> &[bool; 16];
    fn check_key(&self, index: u8) -> bool;
}

impl SDLKeys {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let pump = sdl_context.event_pump().unwrap();
        let mut keys: [bool; 16] = [false; 16];
        Self {event_pump: pump, keys: keys}
    }
}

impl Keys for SDLKeys {
    fn poll_keys(&mut self) -> Option<[bool; 16]> {

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return None,
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return None,
                Event::KeyDown { keycode: e, .. } => match e.unwrap() {
                    Keycode::Num1 => self.keys[0x1] = true,
                    Keycode::Num2 => self.keys[0x2] = true,
                    Keycode::Num3 => self.keys[0x3] = true,
                    Keycode::Num4 => self.keys[0xC] = true,

                    Keycode::Q => self.keys[0x4] = true,
                    Keycode::W => self.keys[0x5] = true,
                    Keycode::E => self.keys[0x6] = true,
                    Keycode::R => self.keys[0xD] = true,

                    Keycode::A => self.keys[0x7] = true,
                    Keycode::S => self.keys[0x8] = true,
                    Keycode::D => self.keys[0x9] = true,
                    Keycode::F => self.keys[0xE] = true,

                    Keycode::Z => self.keys[0xA] = true,
                    Keycode::X => self.keys[0x0] = true,
                    Keycode::C => self.keys[0xB] = true,
                    Keycode::V => self.keys[0xF] = true,

                    _ => {}
                },
                Event::KeyUp { keycode: e, .. } => match e.unwrap() {
                    Keycode::Num1 => self.keys[0x1] = false,
                    Keycode::Num2 => self.keys[0x2] = false,
                    Keycode::Num3 => self.keys[0x3] = false,
                    Keycode::Num4 => self.keys[0xC] = false,

                    Keycode::Q => self.keys[0x4] = false,
                    Keycode::W => self.keys[0x5] = false,
                    Keycode::E => self.keys[0x6] = false,
                    Keycode::R => self.keys[0xD] = false,

                    Keycode::A => self.keys[0x7] = false,
                    Keycode::S => self.keys[0x8] = false,
                    Keycode::D => self.keys[0x9] = false,
                    Keycode::F => self.keys[0xE] = false,

                    Keycode::Z => self.keys[0xA] = false,
                    Keycode::X => self.keys[0x0] = false,
                    Keycode::C => self.keys[0xB] = false,
                    Keycode::V => self.keys[0xF] = false,

                    _ => {}
                },
                _ => {}
            }
        }
        //println!("{:?}", &self.keys);
        Some(self.keys)
    }

    #[inline(always)]
    fn get_keys(&self) -> &[bool; 16] {
        &self.keys
    }

    #[inline(always)]
    fn check_key(&self, index: u8) -> bool {
        self.keys[index as usize]
    }
}