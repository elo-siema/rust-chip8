
extern crate sdl2;

use sdl2::rect::{Point, Rect};
use sdl2::pixels::{Color, PixelFormatEnum};

pub struct SDLDisplay {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    screen: [[u8; 32]; 64],
    draw_flag: bool
}

pub trait Display {
    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool;
    fn clear(&mut self);
    fn render(&mut self);
    //fn po
}

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;
const LWIDTH: usize = 64;
const LHEIGHT: usize = 32;
const SIZE: u32 = 10;//WIDTH as u32 / LWIDTH as u32;

impl SDLDisplay {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo: Window", WIDTH, HEIGHT)
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();
        
        Self {
            canvas: canvas,
            screen: [[0; LHEIGHT]; LWIDTH],
            draw_flag: true
        } 
    }
}

impl Display for SDLDisplay {
    
    fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        let n = sprite.len() as usize;
        let mut yj: usize;
        let mut xi: usize;

        for j in 0..n {
            for i in 0..8 {
                //for wrapping around the edges of the screen:
                yj = (y + j) % LHEIGHT;
                xi = (x + i) % LWIDTH;

                //0x80 - binary 1000 0000
                //shifted right the right amount of bits
                //so we can AND check for collision
                if (sprite[j] & (0x80 >> i)) != 0 {
                    if self.screen[xi][yj] == 1 { collision = true }
                    self.screen[xi][yj] ^= 1;
                }
            }
        }

        self.draw_flag = true;
        collision
    }

    fn clear(&mut self) {
        self.screen = [[0; LHEIGHT]; LWIDTH];
        self.draw_flag = true;
    }

    fn render(&mut self) {
        if(!self.draw_flag) {return;}

        let creator = self.canvas.texture_creator();
        let mut texture = creator
            .create_texture_target(PixelFormatEnum::RGBA8888, LWIDTH as u32, LHEIGHT as u32)
            .unwrap();
        self.canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
            texture_canvas.fill_rect(Rect::new(0, 0, WIDTH as u32, HEIGHT as u32)).unwrap();
        }).unwrap();

        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        self.canvas.clear();
        
        for x in 0..LWIDTH {
            for y in 0..LHEIGHT {
                if self.screen[x][y] > 0 {

                    let rect = Rect::new(x as i32 * SIZE as i32, 
                                         y as i32 * SIZE as i32,
                                         SIZE,
                                         SIZE,);
                    self.canvas
                        .copy_ex(&texture,
                         None,
                         rect,
                         0.0,
                         None,
                         false,
                         false)
                         .unwrap();
                
                }
            }
        }
        self.canvas.present();
        self.draw_flag = false;
    }
}