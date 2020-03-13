use crate::prelude::*;

pub const PLAYER_SPEED_: i32 = 4; // units per tick (must divide GRID (48));
pub const ENEMY_SPEED_: i32 = 2; // units per tick (must divide GRID (48));

/// A Mover is a block that can move over the map.
/// E.g.: the player, crates, ice cubes, ...
#[derive(Debug)]
pub struct Mover {
	pub pos: Pt,       // top-left position, in world coordinates.
	pub inertia_: Dir, // direction currently moving in
	pub intent: Dir,   // direction it wants to move in, if possible TODO: no need to save this
	pub look: Dir,
	pub typ: Block,
}

impl Mover {
	/// New player at given map position.
	pub fn new(pos: Pt, typ: Block) -> Mover {
		Mover::unstage(pos, typ).unwrap()
	}

	/// Used when loading a map: turn blocks into movers.
	pub fn unstage(pos: Pt, typ: Block) -> Option<Mover> {
		assert!(pos.aligned());

		let mut typ = typ;
		let mut intent = Dir::None;
		match typ {
			PLAYER => (),
			CRATE => (),
			ICECUBE => (),
			PIG_L..=PIG_D => {
				intent = LRUD[(typ - PIG_L) as usize];
				typ = PIG;
			}
			_ => return None,
		};

		Some(Mover {
			pos: pos,
			inertia_: Dir::None,
			intent: intent,
			look: Dir::Right,
			typ: typ,
		})
	}

	/// World position of top-left corner.
	pub fn pos(&self) -> Pt {
		self.pos
	}

	pub fn aligned(&self) -> bool {
		self.pos.aligned()
	}

	pub fn grid(&self) -> Pt {
		self.pos.grid()
	}

	pub fn approx_grid(&self) -> Pt {
		self.pos.approx_grid()
	}

	pub fn typ(&self) -> Block {
		self.typ
	}

	pub fn move_intent(&self) -> Dir {
		self.intent
	}

	pub fn inertia(&self) -> Dir {
		self.inertia_
	}

	pub fn speed(&self) -> i32 {
		match self.typ {
			PLAYER => PLAYER_SPEED_,
			PIG => ENEMY_SPEED_,
			_ => PLAYER_SPEED_,
		}
	}

	pub fn texture(&self) -> Tex {
		match self.typ() {
			PIG => (PIG_L as usize) + (self.look as usize) - 1,
			PLAYER => {
				if self.look == Dir::Left {
					PLAYER_L
				} else {
					PLAYER as usize
				}
			}
			_ => self.typ as usize,
		}
	}

	pub fn can_walk(&self, b: Block) -> bool {
		match (self.typ(), b) {
			(CRATE, WATER) => true,
			(ICECUBE, WATER) => true,
			_ => match b {
				NONE => true,
				KEY_B..=KEY_Y => true,
				TOGGLE_OPEN_B..=TOGGLE_OPEN_Y => true,
				BUTTON_B..=BUTTON_Y => true,
				ARROW_L..=ARROW_D => true,
				WATER => false,
				ICE => true,
				EXIT => true,
				_ => false,
			},
		}
	}

	pub fn on_bump(&mut self) {
		match self.typ {
			CRATE | ICECUBE => self.intent = Dir::None,
			PIG => {
				self.intent = {
					self.intent.opposite()
				}
			}
			_ => {}
		}
	}

	pub fn on_bumped(&mut self, d: Dir) {
		match self.typ {
			CRATE | ICECUBE => self.intent = d,
			_ => {}
		}
	}

	pub fn on_align(&mut self) {
		match self.typ {
			CRATE => self.intent = Dir::None,
			_ => {}
		}
	}

	pub fn kill(&mut self) {
		self.typ = NONE;
		assert!(self.is_dead());
	}

	pub fn is_dead(&self) -> bool {
		self.typ == NONE
	}

	pub fn rect(&self) -> Rect {
		Rect::new(self.pos, GRID, GRID)
	}
}
