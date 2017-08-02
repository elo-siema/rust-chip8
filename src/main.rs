mod cpu;
mod keys;
mod display;

extern crate sdl2;

use keys::*;
use std::{thread, time};


fn main() {
    println!("Test");
    let context = sdl2::init().unwrap();
    let mut display = display::SDLDisplay::new(&context);
    let mut keys = keys::SDLKeys::new(&context);
    let mut cpu = cpu::Cpu::new(&mut keys, &mut display);
    

    'main: loop{
        match cpu.poll_keys() {
            Some(e) => {},
            None => break 'main
        }
        
        thread::sleep(time::Duration::from_millis(1));
    }
}