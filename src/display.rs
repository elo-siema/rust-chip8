
extern crate sdl2;

pub struct SDLDisplay {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    screen: [[bool; 32]; 64]
}

pub trait Display {
    fn put_pixel(&mut self, (u8, u8));
    fn render(&self);
    //fn po
}
impl SDLDisplay {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo: Window", 800, 600)
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        Self {
            canvas: canvas,
            screen: [[false; 32]; 64]
        } 
    }
}

impl Display for SDLDisplay {
    
    fn put_pixel(&mut self, coords: (u8, u8)) {
        self.screen[coords.0 as usize][coords.1 as usize] = true;
    }

    fn render(&self) {
    }
}