use crate::prelude::*;

use std::fmt;

/// Dir is a direction.
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Dir {
	None = 0,
	Left = 1,
	Right = 2,
	Up = 3,
	Down = 4,
}

/// All nonzero directions (Left, Right, Up, Down).
pub static LRUD: [Dir; 4] = [Dir::Left, Dir::Right, Dir::Up, Dir::Down];

impl Dir {
	/// Convert direction to vector.
	pub fn vector(self) -> Pt {
		match self {
			Dir::None => Pt(0, 0),
			Dir::Left => Pt(-1, 0),
			Dir::Right => Pt(1, 0),
			Dir::Up => Pt(0, -1),
			Dir::Down => Pt(0, 1),
		}
	}

	/// Keypress corresponding to this direction.
	pub fn key(self) -> Key {
		match self {
			Dir::None => Key::None,
			Dir::Left => Key::Left,
			Dir::Right => Key::Right,
			Dir::Up => Key::Up,
			Dir::Down => Key::Down,
		}
	}

	/// Opposite of this direction.
	pub fn opposite(self) -> Dir {
		match self {
			Dir::None => Dir::None,
			Dir::Left => Dir::Right,
			Dir::Right => Dir::Left,
			Dir::Up => Dir::Down,
			Dir::Down => Dir::Up,
		}
	}
}

impl fmt::Display for Dir {
	fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
		write!(f, "{:?}", self)
	}
}
