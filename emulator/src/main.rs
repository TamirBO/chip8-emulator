use std::env;
use std::fs;
mod cpu;
mod memory;
mod chip8;
mod screen;
mod keys;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
    let mut rom_file = fs::read(&args[1]).unwrap();
    let mut c8 = chip8::Chip8::new(&mut rom_file);


    /*for i in 0..24{
        cpu::step(&mut c8);
        println!("{:x}", c8.cpu.registers[0]);
    }*/

        // The rest of the game loop goes here...


















    /*for i in 0..22{
        cpu::step(&mut c8);
    }
    for (y, row) in c8.screen.data.iter().enumerate(){
        for(x, col) in row.iter().enumerate(){
            if col.to_owned(){
            print!("{} ", 1);
            } 
            else {
            print!("{} ", 0);
        }
    }
        println!();
    }*/



} else {
    println!("Usage: ./chip8 [ROM_NAME]");
}


}







    //c8.memory.show_memory();
    /*for pc in (0x200..0x300).step_by(2){
        cpu::fetch(&c8, pc);
    }*/

    /*let sdl_context = sdl2::init().unwrap();
    let mut display = screen::SdlScreen::new(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        display.canvas.clear();
        display.canvas.present();
        c8.run();
        display.render(&c8.screen);
        // The rest of the game loop goes here...
    }*/
    //c8.run();







