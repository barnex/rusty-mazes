use crate::prelude::*;

// A block in the game map.
pub type Block = u8;

/// If the block is an arrow, return the direction it points in.
pub fn block_arrow_dir(b: Block) -> Dir {
	match b {
		ARROW_D => Dir::Down,
		ARROW_U => Dir::Up,
		ARROW_L => Dir::Left,
		ARROW_R => Dir::Right,
		_ => Dir::None,
	}
}

/// Is the block an arrow?
pub fn block_is_arrow(b: Block) -> bool {
	block_arrow_dir(b) != Dir::None
}

// Human-readable names corresponding to the bitmap files in assets/textures.
pub const NONE: u8 = 0;
pub const BRICK: u8 = 4;
pub const WATER: u8 = 8;
pub const ICE: u8 = 9;
pub const LOCK_B: u8 = 12;
pub const LOCK_G: u8 = 13;
pub const LOCK_R: u8 = 14;
pub const LOCK_Y: u8 = 15;
pub const KEY_B: u8 = 16;
pub const KEY_G: u8 = 17;
pub const KEY_R: u8 = 18;
pub const KEY_Y: u8 = 19;
pub const TOGGLE_CLOSED_B: u8 = 20;
pub const TOGGLE_CLOSED_G: u8 = 21;
pub const TOGGLE_CLOSED_R: u8 = 22;
pub const TOGGLE_CLOSED_Y: u8 = 23;
pub const TOGGLE_OPEN_B: u8 = 24;
pub const TOGGLE_OPEN_G: u8 = 25;
pub const TOGGLE_OPEN_R: u8 = 26;
pub const TOGGLE_OPEN_Y: u8 = 27;
pub const BUTTON_B: u8 = 28;
pub const BUTTON_G: u8 = 29;
pub const BUTTON_R: u8 = 30;
pub const BUTTON_Y: u8 = 31;
pub const ARROW_L: u8 = 32;
pub const ARROW_R: u8 = 33;
pub const ARROW_U: u8 = 34;
pub const ARROW_D: u8 = 35;
pub const PLAYER: u8 = 40;
pub const EXIT: u8 = 41;
pub const CRATE: u8 = 44;
pub const ICECUBE: u8 = 45;
pub const PIG: u8 = 48; // canonical pig, used as type
pub const PIG_L: u8 = 48;
pub const PIG_R: u8 = 49;
pub const PIG_U: u8 = 50;
pub const PIG_D: u8 = 51;

pub const NUM_BLOCKS: usize = 52;

pub const PLAYER_L: Tex = 340;
