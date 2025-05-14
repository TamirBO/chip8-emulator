mod display;
mod input;

use std::env;
use std::fs;
use sdl2;
use sdl2::event::Event;

use chip8::chip8::Chip8;
use chip8::cpu;

use std::time::{Instant, Duration};

use sdl2::keyboard;
use sdl2::keyboard::Keycode;


pub const INSTRUCTIONS_PER_SECOND: u128 = 500;
pub const TIMERS_DECREASE_RATE: u128 = 60;
pub const TIMERS_TICK: u128 = 1_000_000_000 / TIMERS_DECREASE_RATE;
pub const CPU_TICK: u128 = 1_000_000_000 / INSTRUCTIONS_PER_SECOND;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        println!("Usage: {} <rom-file>", args[0]);
        return;
    }
    let mut rom_file = fs::read(&args[1]).unwrap();
    let mut c8 = Chip8::new(&mut rom_file);
    
    let mut now = Instant::now();


    let sdl_context = sdl2::init().unwrap();
    let mut display = display::SdlScreen::new(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        let timer = now.elapsed().as_nanos();

        let keyboard_state = event_pump.keyboard_state();
        input::handle_input(&mut c8.keys, keyboard_state);
        display.render(&c8.screen);

        if timer >= TIMERS_TICK{
            c8.cpu.handle_timers();
            now = Instant::now();        

        }
            if timer >= CPU_TICK{
            cpu::step(&mut c8);
            }

    }
}

