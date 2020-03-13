use crate::encoding;
use crate::prelude::*;
use std::io;
use std::io::Write;
use std::iter::Iterator;
use std::path::PathBuf;

/// Playable game state.
pub struct Gamestate {
	file: PathBuf,
	pub map: Map,       // blocks with fixed location
	pub mv: Vec<Mover>, // blocks with moving location, incl. player
}

impl Gamestate {
	/// new empty gamestate.
	pub fn empty() -> Gamestate {
		Gamestate {
			file: PathBuf::new(),
			map: Map::new(),
			mv: vec![],
		}
	}

	/// Load a map from file (JSON).
	pub fn load(file: PathBuf) -> Result<Gamestate> {
		let staging = encoding::load(&file)?;
		let (map, mv) = encoding::unstage(&staging);
		Ok(Gamestate { file, map, mv })
	}

	/// Handle non-debounced keys (i.e. not for time-critical stuff,
	/// time-critical keys are handled in tick()).
	pub fn handle_key(&mut self, k: Key) {
		match k {
			Key::NextMap => self.next_map(1),
			Key::PrevMap => self.next_map(-1),
			Key::Restart => self.restart_map(),
			_ => (),
		}
	}

	fn restart_map(&mut self) {
		self.try_load_map(self.file.clone());
	}

	/// Jump to the next or previous map,
	/// for delta 1 or -1, respectively.
	fn next_map(&mut self, delta: i32) {
		match encoding::find_next_map(&self.file, delta) {
			Ok(f) => self.try_load_map(f),
			Err(e) => writeln!(io::stderr(), "{}", e).unwrap(),
		}
	}

	fn try_load_map(&mut self, f: PathBuf) {
		match Gamestate::load(f) {
			Ok(g) => *self = g,
			Err(e) => writeln!(io::stderr(), "{}", e).unwrap(),
		}
	}

	/// Advance time by one tick (1/60th of a second).
	/// Player moves according to currently pressed keys.
	pub fn tick(&mut self, keys: KeyStates) {
		// set player move intent according to keys pressed
		self.mv[0].intent = player_move_intent(keys, self.player().pos, self);
		let under = (self.player().pos + Pt(GRID / 2, GRID / 2)) / GRID;
		if self.map[under] != ICE && self.can_move(0, self.player().intent, self.player().speed()) {
			self.try_set_inertia(0, self.mv[0].intent);
		}

		// ask all movers their move intent and try to move along intent,
		// or some other direction dictated by map (Ice, arrows,...).
		for i in 0..self.mv.len() {
			// we can only change inertia if aligned to grid,
			// otherwise we could move off-grid (chaos ensured).
			if self.mv[i].aligned() {
				let intent = self.mv[i].move_intent();
				let blk = self.map[self.mv[i].pos().grid()];
				self.try_set_inertia(
					i,
					match blk {
						ICE => self.mv[i].inertia(), // keep sliding
						ARROW_L..=ARROW_D => match (blk, intent) {
							(ICECUBE, _) => block_arrow_dir(blk),   // pushed by arrow
							(_, Dir::None) => block_arrow_dir(blk), // pushed by arrow
							_ => intent,
						},
						_ => intent, // ok, just move
					},
				);
			}

			// try coasting along inertia.
			// actual movement will trigger triggers.
			self.try_coast(i);
		}

		// triggers may have killed movers, prune them.
		self.prune_killed_movers();

		// finish level should be handled separately, not in the middle of the movers loop
		if self.player().aligned() {
			if self.map[self.player().grid()] == EXIT {
				self.next_map(1);
			}
		}
	}

	fn try_set_inertia(&mut self, i: usize, dir: Dir) {
		if dir == Dir::None {
			if self.mv[i].aligned() {
				self.mv[i].inertia_ = dir
			}
			return;
		}

		let under = self.map[self.mv[i].approx_grid()];
		if alignment_allows_move(self.mv[i].pos, dir) && block_arrow_dir(under) != dir.opposite() {
			self.mv[i].inertia_ = dir;
		}

		if self.mv[i].typ() == ICECUBE {
			self.mv[i].intent = dir
		}

		match (i, dir) {
			(_, Dir::None) => (),
			(0, Dir::Up) | (0, Dir::Down) => (),
			_ => self.mv[i].look = dir,
		}
	}

	// move Mover i along it's inertia direction, if possible.
	// it might get bumped to other directions by the player, ice or arrows.
	fn try_coast(&mut self, i: usize) {
		let dir = self.mv[i].inertia();
		let amount = self.mv[i].speed();

		if dir == Dir::None {
			return;
		}

		// move if possible (not obstructed).
		// else receive an on_bump() callback, which may prompt a different move intent.
		let mut can_move = self.can_move(i, dir, amount);
		if can_move {
			self.do_move(i, dir, amount);
		} else {
			self.mv[i].on_bump();

			// bump while not aligned can be problematic,
			// must allow to change direction immediately, or risk being stuck off-grid.
			if !self.mv[i].aligned() {
				self.try_set_inertia(i, self.mv[i].intent);
			}

			if i == 0 {
				self.process_bump_player(dir, amount);
				can_move = self.can_move(i, dir, amount);
			}

			// bounce back if bumping while on ice
			if !can_move && self.map[self.mv[i].approx_grid()] == ICE {
				self.try_set_inertia(i, self.mv[i].inertia().opposite());
			}
		}
	}

	// move Mover i. Assumes move is possible.
	fn do_move(&mut self, i: usize, dir: Dir, amount: i32) {
		assert!(self.can_move(i, dir, amount));
		self.mv[i].inertia_ = dir;
		self.mv[i].pos += dir.vector() * amount;
		if self.mv[i].pos.aligned() {
			self.process_triggers(i);
			self.mv[i].on_align();
		}
	}

	// player bumps into other Mover.
	// if it's a crate or icecube, will try to push it forward.
	fn process_bump_player(&mut self, dir: Dir, amount: i32) {
		let i = 0; // player

		// which mover did player bump into?
		// call it's on_bump, which may cause it wanting to move.
		// if it wants to move in the bump direction, move it right away,
		// so that it stops blocking the player.
		let probe_point = probe_point(self.mv[i].pos, dir, amount);
		if let Some(j) = self.mover_at(probe_point) {
			if self.mv[j].aligned() {
				self.mv[j].on_bumped(dir); // crates set move_intent on bump
			}

			let intent = self.mv[j].move_intent();
			if intent == dir {
				self.try_set_inertia(j, intent);
				self.try_coast(j);
				self.try_coast(j); // hack for ice+crate, so that player does not bounced back when on ice.
			}
		}

		// try to move the player again, now the the obstruction
		// has potentially been pushed out of the way.
		if self.can_move(i, dir, amount) {
			self.do_move(i, dir, amount)
		}
	}

	// Process triggers for mover i, which must be aligned to grid.
	// This triggers keys, buttons, ...  and causes crates on water to turn into floor.
	fn process_triggers(&mut self, i: usize) {
		assert!(self.mv[i].aligned());

		// only trigger if just moved onto block.
		// do not keep triggering if standing still on block.
		if self.mv[i].inertia() == Dir::None {
			return;
		}

		let grid = self.mv[i].grid();
		let blk = self.map[grid];

		// effect on map (locks, keys, ...)
		match self.map[grid] {
			KEY_B..=KEY_Y => self.trigger_key(grid),
			BUTTON_B..=BUTTON_Y => self.trigger_button(grid),
			_ => (),
		}

		// effect on movers (crates, ice)
		let typ = self.mv[i].typ();
		match (typ, blk) {
			(CRATE, WATER) => {
				self.map.set(grid, NONE);
				self.mv[i].kill();
			}
			(ICECUBE, WATER) => {
				self.map.set(grid, ICE);
				self.mv[i].kill();
			}
			_ => (),
		}
	}

	/// Trigger the key at grid postion pos.
	/// Removes all locks of the same color (inside the action radius).
	fn trigger_key(&mut self, pos: Pt) {
		let lock = self.map[pos] - 4; // lock corresponding to this key. see blocks.rs.
		self.map.set(pos, NONE); // remove key
		self.map.replace(Gamestate::action_range(pos), |b| if b == lock { NONE } else { b });
	}

	/// Trigger the button at grid postion pos.
	/// Toggles the corresponding toggle blocks (inside the action radius).
	fn trigger_button(&mut self, pos: Pt) {
		let open = self.map[pos] - 4; // block corresponding to this button. see blocks.rs.
		let close = self.map[pos] - 8; // block corresponding to this button. see blocks.rs.
		self.map.replace(Gamestate::action_range(pos), |b| {
			if b == open {
				close
			} else if b == close {
				open
			} else {
				b
			}
		});
	}

	fn action_range(center: Pt) -> (Pt, Pt) {
		(center - Pt(32, 32), center + Pt(32, 32))
	}

	fn mover_at(&self, pos: Pt) -> Option<usize> {
		for i in 0..self.mv.len() {
			if self.mv[i].rect().inside(pos) {
				return Some(i);
			}
		}
		None
	}

	pub fn player(&self) -> &Mover {
		&self.mv[0]
	}

	pub fn can_move(&self, i: usize, dir: Dir, amount: i32) -> bool {
		let pos = self.mv[i].pos;

		if dir == Dir::None {
			return true;
		}

		// cannot move if not aligned to grid
		if !alignment_allows_move(pos, dir) {
			return false;
		}

		// edge cannot move into something unwalkable
		let pp = probe_point(pos, dir, amount);
		let blk = self.map[pp / GRID]; // probe pos not neccesarily aligned
		if !self.mv[i].can_walk(blk) {
			return false;
		}
		if dir == block_arrow_dir(blk).opposite() {
			return false;
		}

		// cannot move into other mover (excluding self)
		let probe_rect = Rect::new(pos + dir.vector() * amount, GRID, GRID);
		for m in self.mv.iter().filter(|x| x.pos != pos) {
			if m.rect().overlaps(&probe_rect) {
				return false;
			}
		}

		true
	}

	fn prune_killed_movers(&mut self) {
		let mut i = 1;
		while i < self.mv.len() {
			if self.mv[i].is_dead() {
				self.mv.remove(i);
			} else {
				i += 1;
			}
		}
	}
}

fn probe_point(pos: Pt, dir: Dir, amount: i32) -> Pt {
	pos + dir.vector() * amount
		+ match dir {
			Dir::Right => Pt(GRID - 1, 0),
			Dir::Down => Pt(0, GRID - 1),
			_ => Pt(0, 0),
		}
}

pub fn alignment_allows_move(pos: Pt, dir: Dir) -> bool {
	match dir {
		Dir::None => aligned(pos.x()) && aligned(pos.y()),
		Dir::Left | Dir::Right => aligned(pos.y()),
		Dir::Up | Dir::Down => aligned(pos.x()),
	}
}

/// Given the keys currently pressed, what direction does the player want to move in?
/// This depends on the map. When multiple keys are pressed simultaneously, we attempt
/// to pick a direction that is not blocked, and
/// we pick the direction in which we moved the longest ago.
/// This makes for smooth moves and is quite handy e.g. when walking along a wall
/// and then going through a door: the door won't be missed if a key is released a bit too late.
pub fn player_move_intent(keys: KeyStates, pos: Pt, g: &Gamestate) -> Dir {
	let mut intent = Dir::None;
	for dir in &LRUD {
		let dir = *dir;
		if keys.down[dir.key().id()] {
			intent = dir;
		}
	}

	// refine in case multiple are pressed together
	for dir in &LRUD {
		let dir = *dir;
		if keys.down[dir.key().id()] && !keys.down[dir.opposite().key().id()] && g.can_move(0 /*player*/, dir, PLAYER_SPEED_) {
			intent = dir;
		}
	}
	intent
}
