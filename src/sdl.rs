extern crate sdl2;

use crate::prelude::*;
use sdl2::pixels;
use sdl2::rect;
use sdl2::render::Texture;

/// Display is an abstraction layer over an SDL Canvas and collection of textures,
/// So that none of the game logic needs to be concerned with SDL details.
pub struct Display<'a> {
	canvas: Canvas,
	textures: &'a [Option<Texture<'a>>],
}

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

impl<'a> Display<'a> {
	pub fn new(canvas: Canvas, textures: &'a [Option<Texture<'a>>]) -> Display<'a> {
		Display { canvas, textures }
	}

	pub fn size(&self) -> (i32, i32) {
		let s = self.canvas.output_size().unwrap();
		(s.0 as i32, s.1 as i32)
	}

	pub fn clear(&mut self, c: Color) {
		self.canvas.set_draw_color(sdl_color(c));
		self.canvas.clear();
	}

	pub fn fill_rect(&mut self, c: Color, pos: Pt, w: i32, h: i32) {
		self.canvas.set_draw_color(sdl_color(c));
		self.canvas.fill_rect(rect::Rect::new(pos.0, pos.1, w as u32, h as u32)).unwrap()
	}

	pub fn copy_tex(&mut self, tex: usize, pos: Pt, w: i32, h: i32) {
		let tex = match self.textures[tex].as_ref() {
			Some(t) => t,
			// fall back to texture 0 if not found
			None => self.textures[255].as_ref().unwrap(),
		};
		let dst = Some(rect::Rect::new(pos.0, pos.1, w as u32, h as u32));
		self.canvas.copy_ex(tex, None, dst, 0.0, None, false, false).unwrap()
	}

	pub fn present(&mut self) {
		self.canvas.present()
	}
}

#[inline]
fn sdl_color(c: Color) -> sdl2::pixels::Color {
	pixels::Color::RGBA(c.0, c.1, c.2, c.3)
}
