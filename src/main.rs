mod cpu;
mod keys;
mod display;

extern crate sdl2;

use keys::*;
use std::{thread, time, env, fs, io};
use std::time::{Instant, Duration};
use std::io::Read;
use display::Display;


fn main() {
    println!("Test");
    //init sdl:
    let context = sdl2::init().unwrap();
    let mut display = display::SDLDisplay::new(&context);
    let mut keys = keys::SDLKeys::new(&context);

    //read program:
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let f = fs::File::open(file).expect("file not found");
    let max_size = 3584;
    let mut buf_reader = io::BufReader::with_capacity(max_size, f);
    
    let mut program: Vec<u8> = Vec::with_capacity(max_size);
    match buf_reader.read_to_end(&mut program) {
        Ok(result) => {}
        Err(e) => {
                println!("Error reading file");
                return;
            }
    }

    //init chip8
    let mut cpu = cpu::Cpu::new(&mut keys, &mut display, program);
    let mut timestamp = Instant::now();

    'main: loop{
        match cpu.poll_keys() {
            Some(e) => {},
            None => break 'main
        }

        if timestamp.elapsed() > Duration::from_millis(2) {
            timestamp = Instant::now();
            cpu.tick();
            //print!("tick");
        }

        thread::sleep(time::Duration::from_millis(1));
    }
}