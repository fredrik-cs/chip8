use winit::event::VirtualKeyCode;

// General
pub(crate) const GAMES : [&str; 2] = [
    "tetris.ch8",
    "slipperyslope.ch8"
];
pub(crate) const GAME_NAME : &str = GAMES[1];
pub(crate) const CHIP_WIDTH : u32 = 64;
pub(crate) const CHIP_HEIGHT : u32 = 32;
pub(crate) const CHIP_MEMORY_SIZE : usize = 4096;
pub(crate) const NO_KEY_PRESSED : u8 = 0x10;
pub(crate) const DRAW_KEY_PRESSED : u8 = 0x11;
pub(crate) const KEY_SET : [VirtualKeyCode; 16] = [
    VirtualKeyCode::Key1,
    VirtualKeyCode::Key2,
    VirtualKeyCode::Key3,
    VirtualKeyCode::Key4,
    VirtualKeyCode::Q,
    VirtualKeyCode::W,
    VirtualKeyCode::E,
    VirtualKeyCode::R,
    VirtualKeyCode::A,
    VirtualKeyCode::S,
    VirtualKeyCode::D,
    VirtualKeyCode::F,
    VirtualKeyCode::Z,
    VirtualKeyCode::X,
    VirtualKeyCode::C,
    VirtualKeyCode::V
];

// Sections
pub(crate) const OP_ROUTINE : u16 = 0x0000;
pub(crate) const OP_JUMP : u16 = 0x1000;
pub(crate) const OP_CALL : u16 = 0x2000;
pub(crate) const OP_SKIP_EQUALS_NN : u16 = 0x3000;
pub(crate) const OP_SKIP_NOT_EQUALS_NN : u16 = 0x4000;
pub(crate) const OP_SKIP_EQUALS_XY : u16 = 0x5000;
pub(crate) const OP_SET_VX_NN : u16 = 0x6000;
pub(crate) const OP_ADD_VX_NN : u16 = 0x7000;
pub(crate) const OP_VX_ARITHMETIC : u16 = 0x8000;
pub(crate) const OP_SKIP_NOT_EQUALS_XY : u16 = 0x9000;
pub(crate) const OP_SET_I_NNN : u16 = 0xA000;
pub(crate) const OP_JUMP_NNN_V0 : u16 = 0xB000;
pub(crate) const OP_VX_RANDOM_AND_NN : u16 = 0xC000;
pub(crate) const OP_DRAW : u16 = 0xD000;
pub(crate) const OP_SKIP_ON_INPUT : u16 = 0xE000;
pub(crate) const OP_MISCELLANEOUS: u16 = 0xF000;

// Subsections
pub(crate) const OP_CLEAR_SCREEN : u16 = 0x00E0;
pub(crate) const OP_RETURN : u16 = 0x00EE;
pub(crate) const OP_SET_XY : u16 = 0x0000;
pub(crate) const OP_OR_XY : u16 = 0x0001;
pub(crate) const OP_AND_XY : u16 = 0x0002;
pub(crate) const OP_XOR_XY : u16 = 0x0003;
pub(crate) const OP_ADD_XY : u16 = 0x0004;
pub(crate) const OP_SUBTRACT_XY : u16 = 0x0005;
pub(crate) const OP_SHIFT_RIGHT_XY : u16 = 0x0006;
pub(crate) const OP_REVERSE_SUBTRACT_XY : u16 = 0x0007;
pub(crate) const OP_SHIFT_LEFT_XY : u16 = 0x000E;
pub(crate) const OP_SKIP_INPUT_EQUALS : u16 = 0x009E;
pub(crate) const OP_SKIP_INPUT_NOT_EQUALS : u16 = 0x00A1;
pub(crate) const OP_SET_VX_DELAY : u16 = 0x0007;
pub(crate) const OP_SET_VX_KEY_BLOCKING : u16 = 0x000A;
pub(crate) const OP_SET_DELAY_VX : u16 = 0x0015;
pub(crate) const OP_SET_SOUND_VX : u16 = 0x0018;
pub(crate) const OP_ADD_VX_I : u16 = 0x001E;
pub(crate) const OP_SET_I_CHARACTER : u16 = 0x0029;
pub(crate) const OP_BINARY_DECIMAL : u16 = 0x0033;
pub(crate) const OP_STORE : u16 = 0x0055;
pub(crate) const OP_LOAD : u16 = 0x0065;



