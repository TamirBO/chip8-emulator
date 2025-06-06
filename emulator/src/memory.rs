const MAX_ROM_SIZE: usize = 0xFFF - 0x200;

pub struct Memory{
    pub data: [u8; 4096]
}

impl Memory {
    pub fn new(rom: &mut Vec<u8>) -> Memory {
        let mut data: [u8; 4096] = [0; 4096];
        if rom.len() > MAX_ROM_SIZE {
            panic!("ROM File is too large!")
        }
        let memory_index = 0x200..0xFFF;
        let rom_index = 0..rom.len();
        for (i, j) in memory_index.zip(rom_index){
            data[i] = rom[j];
        }
        for i in 0..FONT_SET.len(){
            data[i] = FONT_SET[i];
        }
        Memory{
            data
        }
    }

    pub fn read_byte(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn read_word(&self, addr: usize) -> u16 {
        ((self.data[addr] as u16) << 8) | ((self.data[addr +1]) as u16)
    }


    //debugging function
    pub fn show_memory(&self){
        println!("{:02x?}", self.data)
    }
}


pub static FONT_SET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];
