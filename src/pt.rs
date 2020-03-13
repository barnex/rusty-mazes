use crate::prelude::*;

use std::fmt;
use std::ops;

/// A 2D point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pt(pub i32, pub i32);

impl Pt {
	#[inline]
	pub fn x(self) -> i32 {
		self.0
	}

	#[inline]
	pub fn y(self) -> i32 {
		self.1
	}

	#[inline]
	pub fn is_neg(self) -> bool {
		self.0 < 0 || self.1 < 0
	}

	#[inline]
	pub fn aligned(self) -> bool {
		aligned(self.0) && aligned(self.1)
	}

	/// Grid position. Panics if not aligned.
	#[inline]
	pub fn grid(self) -> Pt {
		assert!(self.aligned());
		self / GRID
	}

	/// Approximate grid position. OK if not aligned.
	#[inline]
	pub fn approx_grid(self) -> Pt {
		(self + Pt(GRID / 2, GRID / 2)) / GRID
	}
}

impl ops::Index<usize> for Pt {
	type Output = i32;
	fn index(&self, i: usize) -> &i32 {
		match i {
			0 => &self.0,
			1 => &self.1,
			_ => panic!(format!("Pt index out of bounds: {}", i)),
		}
	}
}

impl ops::Add<Pt> for Pt {
	type Output = Pt;
	fn add(self, b: Pt) -> Pt {
		Pt(self.0 + b.0, self.1 + b.1)
	}
}

impl ops::Sub<Pt> for Pt {
	type Output = Pt;
	fn sub(self, b: Pt) -> Pt {
		Pt(self.0 - b.0, self.1 - b.1)
	}
}

impl ops::Mul<i32> for Pt {
	type Output = Pt;
	fn mul(self, b: i32) -> Pt {
		Pt(self.0 * b, self.1 * b)
	}
}

impl ops::Mul<u32> for Pt {
	type Output = Pt;
	fn mul(self, b: u32) -> Pt {
		Pt(self.0 * b as i32, self.1 * b as i32)
	}
}

impl ops::Div<i32> for Pt {
	type Output = Pt;
	fn div(self, b: i32) -> Pt {
		Pt(self.0 / b, self.1 / b)
	}
}

impl ops::Div<u32> for Pt {
	type Output = Pt;
	fn div(self, b: u32) -> Pt {
		Pt(self.0 / b as i32, self.1 / b as i32)
	}
}

impl ops::AddAssign<Pt> for Pt {
	fn add_assign(&mut self, b: Pt) {
		self.0 += b.0;
		self.1 += b.1;
	}
}

impl fmt::Display for Pt {
	fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(f, "({}, {})", self.0, self.1)
	}
}
