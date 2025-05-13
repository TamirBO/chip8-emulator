use sdl2::keyboard::Keycode;



pub struct Keys{
    pub keypad: [bool; 16],
}

impl Keys{
    pub fn new() -> Keys {
        Keys {
            keypad: [false; 16],
        }
    }

    pub fn set_key(&mut self, key: usize, pressed: bool){
        self.keypad[key] = pressed;
        if self.keypad[key]{
        println!("0x{:X} Pressed", key)
        }
    }

}




