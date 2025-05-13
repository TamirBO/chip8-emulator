use chip8::screen;



use sdl2::{self, pixels};
use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCREEN_WIDTH: u32 = 64;
const SCREEN_HEIGHT: u32 = 32;

const SCALE: u32 = 20;
pub struct SdlScreen{
    pub canvas: Canvas<Window>
}

impl SdlScreen{
    pub fn new(sdl_context: &sdl2::Sdl) -> SdlScreen {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "Chip-8 Emulator",
                SCREEN_WIDTH * SCALE,
                SCREEN_HEIGHT * SCALE,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        SdlScreen { canvas: canvas }
    }
    

    pub fn render(&mut self, screen: &screen::Screen){
        for (y, row) in screen.data.iter().enumerate(){
            for (x, &col) in row.iter().enumerate(){
                let x = x as u32 * SCALE;
                let y = y as u32 * SCALE;
                self.canvas.set_draw_color(color(col));
                self.canvas.fill_rect(Rect::new(x as i32, y as i32,SCALE, SCALE)).unwrap();
                
            }
        }
        self.canvas.present();
    }


}



fn color(pixel: bool) -> pixels::Color{
    if pixel{
        Color::RGB(255, 210, 0)
    } else {
        Color::RGB(0, 0, 0)
    }
}


