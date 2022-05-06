use crate::fontset::CHIP8_FONTSET;
use crate::settings::*;
use crate::constants::*;

use std::fs::File;
use std::io::Read;
use rand::Rng;
use std::num::Wrapping;
use crate::constants::{CHIP_WIDTH, CHIP_HEIGHT, CHIP_MEMORY_SIZE};
use std::time::Instant;
use std::cmp::{max, min};
use pixels::Pixels;


pub struct Chip {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    i: u16,
    pub(crate) pc: u16,
    pub(crate) gfx: [u8; (CHIP_WIDTH * CHIP_HEIGHT) as usize],
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16],
    pub(crate) draw_flag: bool,
    pub(crate) pixels: Pixels,
    pub(crate) requested_key: u8
}

impl Chip {

    pub fn initialize(p: Pixels) -> Chip {
        let mut chip = Chip {
            opcode: 0,
            memory: [0; CHIP_MEMORY_SIZE],
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; (CHIP_WIDTH * CHIP_HEIGHT) as usize],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false,
            pixels: p,
            requested_key: NO_KEY_PRESSED
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

    pub fn set_key(&mut self, index: usize, value: u8) {
        self.key[index] = value
    }

    fn get_nd_opcode(&mut self, n: u32) -> u16 {

        let base_dex: u16 = 0x10; let base_bin: u16 = 2;
        let result = (self.opcode & (0xF * base_dex.pow(n - 1))) >> base_bin.pow(n);

        result
    }

    fn get_nd_v(&mut self, n: u32) -> u16 {
        let result = self.v[self.get_nd_opcode(n) as usize] as u16;
        result
    }

    pub fn draw(&mut self) {

        let mut frame = self.pixels.get_frame();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let col = self.gfx[i] * 0xff;
            let rgba =  {
                [col, col, col, 0xff]
            };
            pixel.copy_from_slice(&rgba);
        }
        self.pixels.render();

    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        let pc = self.pc as usize;
        self.draw_flag = false;


        let opcode = (self.memory[pc] as u16) << 8 | (self.memory[pc + 1] as u16);
        self.opcode = opcode;


        // Decode opcode
        println!(" pc = {} | opcode = {:#06x}", self.pc, self.opcode);


        match opcode & 0xF000
        {
            OP_ROUTINE => {
                match opcode & 0x00FF {
                    OP_CLEAR_SCREEN => { // x00E0 : Clear screen
                        for i in 0..(CHIP_WIDTH * CHIP_HEIGHT) {
                            self.gfx[i as usize] = 0;
                        }
                        self.pc += 2;
                    }
                    OP_RETURN => { // x00EE : Return from subroutine
                        self.sp -= 1;
                        self.pc = self.stack[self.sp as usize] + 2;
                    }
                    _ => { // x0NNN : Call machine code routine
                        self.pc += 2;
                    }
                }
            }

            OP_JUMP => { // x1NNN : Jump to address NNN
                self.pc = self.opcode & 0x0FFF
            }

            OP_CALL => { // x2NNN : Call subroutine at NNN
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }

            OP_SKIP_EQUALS_NN => { // x3XNN : Skip next instruction if VX == NN
                if self.get_nd_v(3) == ((opcode & 0x00FF) as u16) { self.pc += 4; }
                else { self.pc += 2;}
            }

            OP_SKIP_NOT_EQUALS_NN => { // x4XNN : Skip next instruction if VX != NN
                if self.get_nd_v(3) != ((opcode & 0x00FF) as u16) { self.pc += 4; }
                else { self.pc += 2;}
            }

            OP_SKIP_EQUALS_XY => { // x5XY0 : Skip next instruction if VX == VY
                if self.get_nd_v(3) == self.get_nd_v(2) { self.pc += 4; }
                else { self.pc += 2;}
            }

            OP_SET_VX_NN => { // x6XNN : Set VX = NN
                self.v[self.get_nd_opcode(3) as usize] = (opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            OP_ADD_VX_NN => { // x7XNN : Add NN to VX
                let index = self.get_nd_opcode(3) as usize;
                self.v[index] = self.v[index].wrapping_add((opcode & 0x00FF) as u8);
                self.pc += 2;
            }

            OP_VX_ARITHMETIC => {
                match opcode & 0x00F {
                    OP_SET_XY => { // x8XY0 : Set VX = VY
                        self.v[self.get_nd_opcode(3) as usize] = self.get_nd_v(2) as u8;
                        self.pc += 2;
                    }
                    OP_OR_XY => { // x8XY1 : Set VX = VX | VY
                        self.v[self.get_nd_opcode(3) as usize] |= self.v[((opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    OP_AND_XY => { // x8XY2 : Set VX = VX & VY
                        self.v[self.get_nd_opcode(3) as usize] &= self.v[((opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    OP_XOR_XY => { // x8XY3 : Set VX = VX ^ VY (xor)
                        self.v[self.get_nd_opcode(3) as usize] ^= self.v[((opcode & 0x00F0) as u16 >> 4) as usize];
                        self.pc += 2;
                    }
                    OP_ADD_XY => { // x8XY4 : Add VY to VX
                        let y = self.get_nd_v(2) as u8;
                        let index_x = self.get_nd_opcode(3) as usize;
                        if y > (0xFF - self.v[index_x]) {
                            self.v[0xF] = 1; //carry
                        }
                        else {
                            self.v[0xF] = 0;
                        }
                        self.v[index_x] = self.v[index_x].wrapping_add(y);
                        self.pc += 2;
                    }
                    OP_SUBTRACT_XY => { // x8XY5 : Subtract VY from VX (+carry)
                        let y = self.get_nd_v(2) as u8;
                        let index_x = self.get_nd_opcode(3) as usize;
                        if y > self.v[index_x] {
                            self.v[0xF] = 1; //carry
                        }
                        else {
                            self.v[0xF] = 0;
                        }
                        self.v[index_x] = self.v[index_x].wrapping_sub(y);
                        self.pc += 2;
                    }
                    OP_SHIFT_RIGHT_XY => { // x8XY6 : Stores the least significant bit of VX in VF and then shifts VX to the right by 1
                        self.v[0xF] = self.get_nd_v(3) as u8 & 0x01;
                        self.v[self.get_nd_opcode(3) as usize] >>= 1;
                        self.pc += 2;
                    }
                    OP_REVERSE_SUBTRACT_XY => { // x8XY7 : Set VX = VY - VX (+carry)
                        let y = self.get_nd_v(2) as u8;
                        let index_x = self.get_nd_opcode(3) as usize;
                        if self.v[index_x] > y {
                            self.v[0xF] = 1; //carry
                        }
                        else {
                            self.v[0xF] = 0;
                        }
                        self.v[index_x] = y.wrapping_sub(self.v[index_x]);
                        self.pc += 2;
                    }
                    OP_SHIFT_LEFT_XY => { // x8XYE : Stores the most significant bit of VX in VF and then shifts VX to the left by 1
                        self.v[0xF] = self.get_nd_v(3) as u8 & 0x80;
                        self.v[self.get_nd_opcode(3) as usize] <<= 1;
                        self.pc += 2;
                    }
                    _ => {
                        println!("Unknown opcode: {}\n", opcode);
                    }
                }
            }

            OP_SKIP_NOT_EQUALS_XY => { // x9XY0 : Skip next instruction if VX != VY
                if self.get_nd_v(3) != self.get_nd_v(2) { self.pc += 4; }
                else { self.pc += 2;}
            }

            OP_SET_I_NNN => { // xANNN : Set i to address NNN
                self.i = opcode & 0x0FFF;
                self.pc += 2;
            }

            OP_JUMP_NNN_V0 => { // xBNNN : Jumps to the address NNN plus V0
                self.pc = (opcode & 0x0FFF) + (self.v[0] as u16);
            }

            OP_VX_RANDOM_AND_NN => { // xCXNN : Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN
                let num = rand::thread_rng().gen_range(0..256);
                self.v[self.get_nd_opcode(3) as usize] = (num & opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            OP_DRAW => { // xDXYN : Draw at (VX, VY) with height of N and width of 8
                let x: u16 = self.get_nd_v(3) as u16;
                let y: u16 = self.get_nd_v(2) as u16;
                let height: u16 = opcode & 0x000F;
                let mut pixel: u16;

                self.v[0xF as usize] = 0;
                for yline in 0..height
                {
                    pixel = self.memory[(self.i + yline) as usize] as u16;
                    for xline in 0..8
                    {
                        if (pixel & (0x80 >> xline)) != 0
                        {
                            if self.gfx[(x + xline + (y + yline) * 64) as usize] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[(x + xline + ((y + yline) * 64)) as usize] ^= 1;
                        }
                    }
                }

                self.draw_flag = true;
                self.pc += 2;
            }

            OP_SKIP_ON_INPUT => {
                match opcode & 0x00FF {
                    OP_SKIP_INPUT_EQUALS => { // xEX9E : Skip next instruction if key() == VX
                        if self.key[self.get_nd_v(3) as usize] != 0 { self.pc += 4; }
                        else { self.pc += 2; }
                    }
                    OP_SKIP_INPUT_NOT_EQUALS => { // xEXA1 : Skip next instruction if key() != VX
                        if self.key[self.get_nd_v(3) as usize] == 0 { self.pc += 4; }
                        else { self.pc += 2; }
                    }
                    _ => {
                        println!("Unknown opcode: {}\n", opcode);
                    }
                }
            }

            OP_MISCELLANEOUS => {
                match opcode & 0x00FF {
                    OP_SET_VX_DELAY => { // xFX07 : Set VX to delay_timer
                        self.v[self.get_nd_opcode(3) as usize] = self.delay_timer;
                        self.pc += 2;
                    }
                    OP_SET_VX_KEY_BLOCKING => { // xFX0A : [BLOCKING] Set VX to pressed key
                        for i in 0..16 {
                            if self.key[i] != 0 {
                                self.requested_key = i as u8;
                                self.v[self.get_nd_opcode(3) as usize] = self.requested_key;
                                break;
                            }
                        }
                    }
                    OP_SET_DELAY_VX => { // xFX15 : Set delay_timer to VX
                        self.delay_timer = self.get_nd_v(3) as u8;
                        self.pc += 2;
                    }
                    OP_SET_SOUND_VX => { // xFX18 : Set sound_timer to VX
                        self.sound_timer = self.get_nd_v(3) as u8;
                        self.pc += 2;
                    }
                    OP_ADD_VX_I => { // xFX1E : Adds VX to i, no carry
                        self.i += self.get_nd_v(3) as u16;
                        self.pc += 2;
                    }
                    OP_SET_I_CHARACTER => { // xFX29 : Sets i to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font
                        self.i = CHIP8_FONTSET[self.get_nd_v(3) as usize] as u16;
                        self.pc += 2;
                    }
                    OP_BINARY_DECIMAL => { // xFX33 : Binary coded decimal of X memory i-i+2
                        self.memory[self.i as usize]       = (self.get_nd_v(3) / 100) as u8;
                        self.memory[(self.i + 1) as usize] = ((self.get_nd_v(3) / 10) % 10) as u8;
                        self.memory[(self.i + 2) as usize] = ((self.get_nd_v(3) % 100) % 10) as u8;
                        self.pc += 2;
                    }
                    OP_STORE => { // xFX55 : Stores from V0 to VX (including VX) in memory, starting at address i.
                                // The offset from i is increased by 1 for each value written, but i itself is left unmodified
                        let index = self.i;
                        for i in 0..=self.get_nd_opcode(3) {
                            self.memory[(index + i) as usize] = self.v[i as usize];
                        }
                        self.pc += 2;
                    }
                    OP_LOAD => { // xFX65 : Fills from V0 to VX (including VX) with values from memory, starting at address i.
                                // The offset from i is increased by 1 for each value written, but i itself is left unmodified
                        let index = self.i;
                        for i in 0..=self.get_nd_opcode(3) {
                            self.v[i as usize] = self.memory[(index + i) as usize];
                        }
                        self.pc += 2;
                    }
                    _ => {
                        println!("Unknown opcode: {}\n", opcode);
                    }
                }
            }

            _ => {
                println!("Unknown opcode: {}\n", opcode);
            }
        }

        // Update timers
        if self.delay_timer > 0 { self.delay_timer -= 1; }

        if self.sound_timer > 0
        {
            println!("BEEP!\n");
            self.sound_timer -= 1;
        }
    }

}