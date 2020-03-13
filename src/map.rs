use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops;
use std::result;

/// Infinite 2D array of blocks.
#[derive(Serialize, Deserialize)]
pub struct Map {
	pub blocks: Vec<Vec<Block>>,
}

impl Map {
	/// New empty map.
	pub fn new() -> Map {
		Map { blocks: Vec::new() }
	}

	/// Set block at position p.
	/// p must be strictly positive.
	pub fn set(&mut self, p: Pt, b: Block) {
		let (x, y) = (p.0, p.1);
		if x < 0 || y < 0 {
			panic!("Map.set: Pt out of bounds: {:?}", p);
		}
		let x = x as usize;
		let y = y as usize;

		if y >= self.blocks.len() {
			self.blocks.reserve(y - self.blocks.len() + 1);
			while y >= self.blocks.len() {
				self.blocks.push(Vec::new());
			}
		}
		if x >= self.blocks[y].len() {
			self.blocks.reserve(x - self.blocks[y].len() + 1);
			while x >= self.blocks[y].len() {
				self.blocks[y].push(0);
			}
		}
		self.blocks[y][x] = b;
	}

	pub fn replace<F: Fn(Block) -> Block>(&mut self, range: (Pt, Pt), f: F) {
		for y in (range.0).1..(range.1).1 {
			for x in (range.0).0..(range.1).0 {
				let p = Pt(x, y);
				let orig = self[p];
				let new = f(orig);
				if new != orig {
					self.set(p, new);
				}
			}
		}
	}
}

impl ops::Index<Pt> for Map {
	type Output = Block;
	fn index(&self, p: Pt) -> &Block {
		let (x, y) = (p.0, p.1);

		// disallow 0 so that we can never bump into negative positions
		// (where pos / GRID does not simply give the grid cell).
		if x <= 0 || y <= 0 {
			return &1; // Block outside of map
		}
		let x = x as usize;
		let y = y as usize;

		if y >= self.blocks.len() {
			return &0;
		}
		if x >= self.blocks[y].len() {
			return &0;
		}

		&self.blocks[y][x]
	}
}

impl fmt::Display for Map {
	fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
		for row in &self.blocks {
			write!(f, "|")?;
			for b in row {
				write!(f, "{} ", b)?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}
