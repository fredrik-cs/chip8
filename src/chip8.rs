use crate::fontset::CHIP8_FONTSET;
use crate::settings::*;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use rand::prelude::*;
use rand::Rng;
use std::num::Wrapping;


pub struct Chip {
    opcode: u16,
    memory: [u8; 4096],
    V: [u8; 16],
    I: u16,
    pc: u16,
    pub(crate) gfx: [u8; 64*32],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16],
    pub(crate) draw_flag: bool
}

impl Chip {

    pub fn initialize() -> Chip {
        let mut chip = Chip {
            opcode: 0,
            memory: [0; 4096],
            V: [0; 16],
            I: 0,
            pc: 0x200,
            gfx: [0; 64*32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false
        };
        for i in 0..80 {
            chip.memory[i] = CHIP8_FONTSET[i];
        }
        chip
    }

    pub fn load_program(&mut self, mut file: File) {
        let mut buffer: Vec<u8> = Vec::new();
        let buffer_size = file.read_to_end(&mut buffer).unwrap();

        for i in 0..buffer_size {
            self.memory[512 + i] = buffer[i];
        }
    }

    pub fn set_keys(&mut self) {

    }

    fn randomize_gfx(&mut self) {
        for i in 0..64*32 {

        }
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = ((i % WIDTH as usize) / (SCALE as usize)) as i16;
            let y = ((i / WIDTH as usize) / (SCALE as usize)) as i16;

            //println!("{}", self.gfx.iter().sum::<u8>());
            let col = self.gfx[(x + 64*y) as usize] * 255;
            let rgba =  {
                [col, col, col, 0xff]
            };

            // let rgba =  {
            //     [0x99, 0x66, 0x33, 0xff]
            // };

            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        self.opcode = ((self.memory[self.pc as usize] as u16) << 8 | (self.memory[(self.pc + 1) as usize] as u16));

        // Decode opcode
        //println!("pc = {} | opcode = {:#06x}", self.pc, self.opcode);

        match self.opcode & 0xF000
        {
            0x0000 => {
                match self.opcode & 0x00FF {
                    0x00E0 => { // x00E0 : Clear screen

                    }
                    0x00EE => { // x00EE : Return from subroutine
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize] + 2;
                    }
                    _ => { // x0NNN : Call machine code routine

                    }
                }
            }

            0x1000 => { // x1NNN : Jump to address NNN
                self.pc = self.opcode & 0x0FFF
            }

            0x2000 => { // x2NNN : Call subroutine at NNN
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }

            0x3000 => { // x3XNN : Skip next instruction if VX == NN
                if (self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] as u16) == self.opcode & 0x00FF { self.pc += 4; }
                else { self.pc += 2;}
            }

            0x4000 => { // x4XNN : Skip next instruction if VX != NN
                if (self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] as u16) != self.opcode & 0x00FF { self.pc += 4; }
                else { self.pc += 2;}
            }

            0x5000 => { // x5XY0 : Skip next instruction if VX == VY
                if self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] == self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize] { self.pc += 4; }
                else { self.pc += 2;}
            }

            0x6000 => { // x6XNN : Set VX = NN
                self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] = ((self.opcode & 0x00FF) as u8);
                self.pc += 2;
            }

            0x7000 => { // x7XNN : Add NN to VX
                //println!("{} + {} = {}", self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize], ((self.opcode & 0x00FF) as u8), 1);
                let new = Wrapping(self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize]) + Wrapping(((self.opcode & 0x00FF) as u8));
                self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] = new.0;
                self.pc += 2;
            }

            0x8000 => {
                match self.opcode & 0x00F {
                    0x0000 => { // x8XY0 : Set VX = VY
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] = self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    0x0001 => { // x8XY1 : Set VX = VX | VY
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] |= self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    0x0002 => { // x8XY2 : Set VX = VX & VY
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] &= self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    0x0003 => { // x8XY3 : Set VX = VX ^ VY (xor)
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] ^= self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    0x0004 => { // x8XY4 : Add VY to VX
                        if self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize] > (0xFF - self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize]) {
                            self.V[0xF] = 1; //carry
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] += self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    0x0005 => { // x8XY5 : Subtract VY to VX (+carry)
                        if self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize] > (self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize]) {
                            self.V[0xF] = 1; //carry
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] += self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    0x0006 => { // x8XY6 : Stores the least significant bit of VX in VF and then shifts VX to the right by 1
                        self.V[0xF] = self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] & 0x01;
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] >>= 1;
                        self.pc += 2;
                    }
                    0x0007 => { // x8XY7 : Set VX = VY - VX (+carry)
                        if self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] > (self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize]) {
                            self.V[0xF] = 1; //carry
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] =
                            self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize] - self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize];
                        self.pc += 2;
                    }
                    0x000E => { // x8XYE : Stores the most significant bit of VX in VF and then shifts VX to the left by 1
                        self.V[0xF] = self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] & 0x80;
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] <<= 1;
                        self.pc += 2;
                    }
                    _ => { // TODO: Check what to do here
                        println!("Unknown opcode: {}\n", self.opcode);
                    }
                }
            }

            0x9000 => { // x9XY0 : Skip next instruction if VX != VY
                if self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] != self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize] { self.pc += 4; }
                else { self.pc += 2;}
            }

            0xA000 => { // xANNN : Set I to address NNN
                self.I = self.opcode & 0x0FFF;
                self.pc += 2;
            }

            0xB000 => { // xBNNN : Jumps to the address NNN plus V0
                self.pc = (self.opcode & 0x0FFF) + (self.V[0] as u16);
            }

            0xC000 => { // xCXNN : Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN
                let num = rand::thread_rng().gen_range(0..256);
                self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] = (num & self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            0xD000 => { // xDXYN : Draw at (VX, VY) with height of N and width of 8
                let x: u16 = self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] as u16;
                let y: u16 = self.V[((self.opcode & 0x00F0) as u16 >> 4) as usize] as u16;
                let height: u16 = self.opcode & 0x000F;
                let mut pixel: u16;

                self.V[0xF as usize] = 0;
                for yline in 0..height
                {
                    pixel = self.memory[(self.I + yline) as usize] as u16;
                    for xline in 0..8
                    {
                        if (pixel & (0x80 >> xline)) != 0
                        {
                            if self.gfx[(x + xline + (y + yline) * 64) as usize] == 1 {
                                self.V[0xF as usize] = 1;
                            }
                            self.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                        }
                    }
                }

                self.draw_flag = true;
                self.pc += 2;
            }

            0xE000 => {
                match self.opcode & 0x00FF {
                    0x009E => { // xEX9E : Skip next instruction if key() == VX
                        if self.key[self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] as usize] != 0 { self.pc += 4; }
                        else { self.pc += 2;}
                    }
                    0x00A1 => { // xEX9E : Skip next instruction if key() != VX
                        if self.key[self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] as usize] == 0 { self.pc += 4; }
                        else { self.pc += 2;}
                    }
                    _ => {  // TODO: Check what to do here
                        println!("Unknown opcode: {}\n", self.opcode);
                    }
                }
            }

            0xF000 => {
                match self.opcode & 0x00FF {
                    0x0007 => { // xFX07 : Set VX to delay_timer
                        self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] = self.delay_timer;
                        self.pc += 2;
                    }
                    0x000A => { // xFX0A : [BLOCKING] Set VX to pressed key

                    }
                    0x0015 => { // xFX15 : Set delay_timer to VX
                        self.delay_timer = self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize];
                        self.pc += 2;
                    }
                    0x0018 => { // xFX18 : Set sound_timer to VX
                        self.sound_timer = self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize];
                        self.pc += 2;
                    }
                    0x001E => { // xFX1E : Adds VX to I, no carry
                        self.I += self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] as u16;
                        self.pc += 2;
                    }
                    0x0029 => { // xFX29 : Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font
                        self.sound_timer = self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize];
                        self.pc += 2;
                    }
                    0x0033 => { // xFX33 : Binary coded decimal of X memory I-I+2
                        self.memory[self.I as usize]     = self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] / 100;
                        self.memory[(self.I + 1) as usize] = (self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] / 10) % 10;
                        self.memory[(self.I + 2) as usize] = (self.V[((self.opcode & 0x0F00) as u16 >> 8) as usize] % 100) % 10;
                        self.pc += 2;
                    }
                    0x0055 => { // xFX55 : Stores from V0 to VX (including VX) in memory, starting at address I.
                                // The offset from I is increased by 1 for each value written, but I itself is left unmodified
                        let index = self.I;
                        for i in 0..=((self.opcode & 0x0F00) as u16 >> 8) {
                            self.memory[(index + i) as usize] = self.V[i as usize];
                        }
                        self.pc += 2;
                    }
                    0x0065 => { // xFX65 : Fills from V0 to VX (including VX) with values from memory, starting at address I.
                                // The offset from I is increased by 1 for each value written, but I itself is left unmodified
                        let index = self.I;
                        for i in 0..=((self.opcode & 0x0F00) as u16 >> 8) {
                            self.V[i as usize] = self.memory[(index + i) as usize];
                        }
                        self.pc += 2;
                    }
                    _ => {  // TODO: Check what to do here
                        println!("Unknown opcode: {}\n", self.opcode);
                    }
                }
            }

            _ => { // TODO: Check what to do here
                println!("Unknown opcode: {}\n", self.opcode);
            }
        }

        // Update timers
        if self.delay_timer > 0 { self.delay_timer -= 1; }

        if self.sound_timer > 0
        {
            if self.sound_timer == 1 { println!("BEEP!\n"); }
            self.sound_timer -= 1;
        }

        //println!("BOOP!\n");
    }

}