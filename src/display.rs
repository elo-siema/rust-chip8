
extern crate sdl2;

pub struct SDLDisplay {
    canvas: sdl2::render::Canvas<sdl2::video::Window>
}

pub trait Display {
    fn put_pixel(&self, (u8, u8));
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

        Self {canvas: canvas} 
    }
}

impl Display for SDLDisplay {
    
    fn put_pixel(&self, coords: (u8, u8)) {

    }

    fn render(&self) {
    }
}