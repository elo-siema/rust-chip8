mod cpu;
mod keys;
mod display;

extern crate sdl2;


fn main() {
    println!("Test");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Window", 800, 600)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let mut tick = 0;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main: loop{
        'event: loop{

        }
    }
}