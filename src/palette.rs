use crate::prelude::*;

/// The Palette allows the user to pick bloks in edit mode.
pub struct Palette {
	pub screen_pos: Pt,
	pub columns: i32,
	pub grid_px: i32,
	pub height: i32,

	pub buttons: Vec<Tex>,
	pub selected: usize,
}

impl Palette {
	pub fn new(screen_pos: Pt, columns: i32, grid_px: i32, height: i32, buttons: Vec<Tex>) -> Palette {
		Palette {
			screen_pos,
			columns,
			grid_px,
			height,
			buttons,
			selected: 0,
		}
	}

	pub fn width(&self) -> i32 {
		self.columns * self.grid_px
	}

	pub fn button_click(&self, pos: Pt, left: bool, right: bool) -> Option<usize> {
		// position in internal grid
		let pos = (pos - self.screen_pos) / self.grid_px;
		if pos.is_neg() {
			return None;
		}

		if left {
			let button = pos.x() as usize + pos.y() as usize * self.columns as usize;
			if button >= self.buttons.len() {
				None
			} else {
				Some(button)
			}
		} else {
			None
		}
	}

	// tests wheter a mouse position is inside this Pane.
	pub fn is_inside(&self, pos: Pt) -> bool {
		let min = self.screen_pos;
		let max = self.screen_pos + Pt(self.width() as i32, self.height as i32);
		pos.0 >= min.0 && pos.1 >= min.1 && pos.0 <= max.0 && pos.1 <= max.1
	}

	pub fn render(&self, disp: &mut Display) {
		disp.fill_rect(Color(128, 128, 128, 128), self.screen_pos, self.width() + 1, self.height);
		let s = self.grid_px;
		for (i, b) in self.buttons.iter().enumerate() {
			// Clear background first
			// only needed for non-opaque sprites.
			disp.copy_tex(0, self.button_pos(i), s, s);
			if *b != 0 {
				disp.copy_tex(*b, self.button_pos(i), s, s);
			}
		}
		disp.copy_tex(BUTTON_SELECTED, self.button_pos(self.selected), s, s)
	}

	// Position on the display, in pixels, of the i'th button.
	pub fn button_pos(&self, i: usize) -> Pt {
		let i = i as i32;
		Pt(i % self.columns as i32, i / self.columns as i32) * self.grid_px + self.screen_pos
	}
}

// texture to render over selected button.
const BUTTON_SELECTED: usize = 1000;
