
use sdl2::{self, pixels};
use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

const SCALE: u32 = 20;
pub struct Screen{
   pub data: [[bool; 64]; 32],
}

impl Screen {
    pub fn new() -> Screen {
        Screen{
            data: [[false; 64]; 32]
        }
    }
}


