use crate::prelude::*;

/// The Viewport represents the window through which the player looks at the world.
pub struct Viewport {
	center: Pt, // point in world where viewport centers on. e.g. player.
	zoom_level: usize,
	canvas_w: i32,
	canvas_h: i32,
}

// Allowed zoom levels (multiplier, divider). These play well with nice composite sprite sizes.
static ZOOM_LEVELS: [(i32, i32); 9] = [(1, 8), (1, 6), (1, 4), (1, 3), (1, 2), (1, 1), (2, 1), (3, 1), (4, 1)];

impl Viewport {
	/// Construct a Viewport for canvas size (in pixels).
	/// The size can later be changed later with set_canvas_size().
	pub fn new(canvas_w: i32, canvas_h: i32) -> Viewport {
		Viewport {
			center: Pt(0, 0),
			zoom_level: 5,
			canvas_w: canvas_w,
			canvas_h: canvas_h,
		}
	}

	/// Change the canvas size (in pixels).
	/// Used, e.g., after a window resize.
	pub fn set_canvas_size(&mut self, width: i32, height: i32) {
		self.canvas_w = width;
		self.canvas_h = height;
	}

	/// Center the viewport around a point in the world (typically the player's avatar).
	pub fn set_center(&mut self, center: Pt) {
		self.center = center;
	}

	/// Increase zoom level, if not at maximum yet.
	/// See static ZOOM_LEVELS.
	pub fn zoom_in(&mut self) {
		if self.zoom_level < ZOOM_LEVELS.len() - 1 {
			self.zoom_level += 1;
		}
	}

	/// Decrease zoom level, if not at minimum yet.
	/// See static ZOOM_LEVELS.
	pub fn zoom_out(&mut self) {
		if self.zoom_level > 0 {
			self.zoom_level -= 1;
		}
	}

	/// Move the viewport. Relative motion is bigger when zoomed out.
	/// Used, e.g., for looking around in edit mode.
	pub fn pan_view(&mut self, delta: Pt) {
		let multiplier = (GRID * self.zoom().1) / 2; // pan more if zoomed out
		self.center += delta * multiplier;
	}

	/// Convert a position on the canvas to a position in the game world.
	/// Used, e.g., to determine the world postion of mouse clicks.
	pub fn to_world(&self, canvas: Pt) -> Pt {
		let (w, h) = (self.canvas_w, self.canvas_h);
		((canvas - Pt(w, h) / 2) * self.zoom().1) / self.zoom().0 + self.center
	}

	/// Render the map as seen through the viewport.
	pub fn render_map(&self, disp: &mut Display, map: &Map) {
		// hack: speed-up zoomed-out (slow) view by not rendering background blocks
		// thus, must clear background first.
		let zoomed = { self.zoom().1 > 1 };
		if zoomed {
			disp.clear(Color(255, 255, 255, 255));
		}

		// range of blocks visible in viewport
		let s = (GRID * self.zoom().0) / self.zoom().1;
		let (min, max) = self.view_rect(disp);
		let min = min / GRID - Pt(1, 1);
		let max = max / GRID + Pt(1, 1);

		for y in min.1..max.1 {
			for x in min.0..max.0 {
				let p_grid = Pt(x, y);
				let p_world = p_grid * GRID;
				let blk = map[p_grid];

				// clear background first (only really needed for non-opaque sprites).
				// hack: skip if zoomed out (slow)
				if !zoomed {
					disp.copy_tex(0, self.to_canvas(p_world), s, s);
				}
				if blk != 0 {
					disp.copy_tex(blk as usize, self.to_canvas(p_world), s, s);
				}
			}
		}
	}

	pub fn render_movers(&self, disp: &mut Display, m: &[Mover]) {
		let s = (GRID * self.zoom().0) / self.zoom().1;
		for m in m {
			disp.copy_tex(m.texture(), self.to_canvas(m.pos()), s, s);
		}
	}

	// The current zoom multiplier and divider.
	// Usage: scale by multiplying first, then dividing.
	fn zoom(&self) -> (i32, i32) {
		ZOOM_LEVELS[self.zoom_level]
	}

	// Convert a point in the world to a point on the canvas (i.e. in pixels).
	fn to_canvas(&self, world: Pt) -> Pt {
		let (w, h) = (self.canvas_w, self.canvas_h);
		((world - self.center) * self.zoom().0) / self.zoom().1 + Pt(w, h) / 2
	}

	// Top-left and bottom-right world coordinates of viewport window.
	// Used to caclulate which sprites are visible.
	fn view_rect(&self, disp: &Display) -> (Pt, Pt) {
		let (width, height) = disp.size();
		let z = self.zoom();

		let width = (width * z.1) / z.0;
		let height = (height * z.1) / z.0;

		let min = self.center - Pt(width / 2, height / 2);
		let max = self.center + Pt(width / 2, height / 2);

		(min, max)
	}
}
