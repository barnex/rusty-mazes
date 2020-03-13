use crate::encoding;
use crate::palette::*;
use crate::prelude::*;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

// Size, in quanta, of a block. Ideally equal to sprite size in pixels.
pub const GRID: i32 = 48;

/// Editor allows the user to draw and play maps.
pub struct Editor {
	file: PathBuf,        // current map file
	staging: Map,         // current map, edit mode
	viewport: Viewport,   // visible portion of map
	paused: bool,         // editing or playing
	palette: Palette,     // visible while editing
	gamestate: Gamestate, // current map, playing mode
}

impl Editor {
	/// Load file for editing.
	pub fn new(width: i32, height: i32, file: PathBuf, paused: bool) -> Editor {
		let mut viewport = Viewport::new(width, height);
		viewport.set_center(Pt(width / 2 - 4 * GRID, height / 2));

		if !file.exists() {
			try_create_empty_map(&file)
		}

		let gamestate = if paused {
			Gamestate::empty()
		} else {
			Gamestate::load(file.clone()).expect("loading map")
		};

		Editor {
			palette: Palette::new(Pt(0, 0), 4, GRID, 2222, (0..NUM_BLOCKS).into_iter().collect()),
			viewport,
			paused,
			staging: encoding::load(&file).expect("loading map"),
			gamestate,
			file,
		}
	}

	/// Called by main loop every 16.6ms (on average) to advance time.
	/// If the rendering stalls to < 60 FPS, tick is
	/// called multiple times to make up for the dropped frames.
	pub fn tick(&mut self, keys: KeyStates) {
		if self.paused {
			return;
		}
		self.gamestate.tick(keys);
		self.viewport.set_center(self.gamestate.player().pos() + Pt(GRID / 2, GRID / 2));
	}

	/// render Editor to display.
	pub fn render(&self, disp: &mut Display) {
		match self.paused {
			true => self.render_paused(disp),
			false => self.render_playing(disp),
		}
	}

	fn render_playing(&self, disp: &mut Display) {
		self.viewport.render_map(disp, &self.gamestate.map);
		self.viewport.render_movers(disp, &self.gamestate.mv);
	}

	fn render_paused(&self, disp: &mut Display) {
		self.viewport.render_map(disp, &self.staging);
		self.palette.render(disp);
	}

	pub fn handle_resize(&mut self, x: i32, y: i32) {
		self.viewport.set_canvas_size(x, y);
	}

	pub fn handle_mouse(&mut self, pos: Pt, left: bool, right: bool) {
		if self.palette.is_inside(pos) {
			self.handle_click_palette(self.palette.button_click(pos, left, right))
		} else {
			self.handle_mouse_viewport(pos, left, right)
		}
	}

	// called when user picks a block from the blocks palette.
	fn handle_click_palette(&mut self, button: Option<usize>) {
		if let Some(button) = button {
			self.palette.selected = button
		}
	}

	fn handle_mouse_viewport(&mut self, pos: Pt, left: bool, right: bool) {
		if !self.paused {
			return;
		}

		let pos = self.viewport.to_world(pos) / GRID;
		if pos.is_neg() {
			return;
		}
		if right {
			self.staging.set(pos, NONE)
		}
		if left {
			self.staging.set(pos, self.palette.selected as Block)
		}
	}

	pub fn handle_mouse_wheel(&mut self, x: i32, y: i32) {
		self.pan_view(Pt(-x, -y))
	}

	pub fn handle_key(&mut self, k: Key) {
		match k {
			// Pause-independent keys:
			Key::Pause => self.toggle_pause(),
			Key::ZoomIn => self.viewport.zoom_in(),
			Key::ZoomOut => self.viewport.zoom_out(),
			// Pause-dependent keys:
			_ => match self.paused {
				true => self.handle_key_paused(k),
				false => self.gamestate.handle_key(k),
			},
		}
	}

	fn handle_key_paused(&mut self, k: Key) {
		match k {
			Key::Left => self.pan_view(Pt(-2, 0)),
			Key::Right => self.pan_view(Pt(2, 0)),
			Key::Down => self.pan_view(Pt(0, 2)),
			Key::Up => self.pan_view(Pt(0, -2)),
			Key::Save => self.try_save(),
			_ => (),
		}
	}

	fn try_save(&self) {
		let f = &self.file;
		if let Err(e) = self.save(f) {
			writeln!(io::stderr(), "Error saving map to {}: {}", &f.to_string_lossy(), e).unwrap();
		}
	}

	fn save(&self, p: &Path) -> Result<()> {
		encoding::save(&self.staging, p)
	}

	pub fn load(&mut self, p: PathBuf) -> Result<()> {
		self.gamestate = Gamestate::load(p)?;
		Ok(())
	}

	fn toggle_pause(&mut self) {
		if self.paused {
			self.save(&self.file).expect("saving map");
			self.gamestate = Gamestate::load(self.file.clone()).expect("loading map");
		}
		self.paused = !self.paused;
	}

	// move the viewport by a relative amount.
	fn pan_view(&mut self, delta: Pt) {
		self.viewport.pan_view(delta)
	}
}

fn try_create_empty_map(f: &Path) {
	println!("creating new map: '{}'", f.to_string_lossy());
	encoding::save(&Map::new(), f).expect("error creating new map")
}

#[inline]
pub fn aligned(x: i32) -> bool {
	(x % GRID) == 0
}