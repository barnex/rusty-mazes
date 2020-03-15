extern crate sdl2;
use game::encoding::*;
use game::prelude::*;

use sdl2::event;
use sdl2::event::Event;
use sdl2::mouse;
use sdl2::render::Texture;
use sdl2::EventPump;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;
use std::time;

type Canvas = sdl2::render::Canvas<sdl2::video::Window>;

pub fn main() {
	if let Err(e) = main_() {
		io::stderr().write_all(&e.to_string().into_bytes()).unwrap();
		process::exit(1);
	}
}

pub fn main_() -> Result<()> {
	let tex_dir = "assets/textures";
	let map_dir = "assets/maps";

	let (canvas, mut event_pump) = init_window()?;
	let tex_creator = canvas.texture_creator();

	// load all <number>.bmp files as textures.
	let mut textures = Vec::<Option<Texture>>::new();
	for f in fs::read_dir(tex_dir)? {
		let f = f?.path();
		if let Some(ext) = f.extension() {
			let ext = ext.to_string_lossy();
			if ext == "bmp" {
				if let Some(stem) = f.file_stem() {
					if let Ok(n) = stem.to_string_lossy().parse::<usize>() {
						let surf = sdl2::surface::Surface::load_bmp(f)?;
						while n >= textures.len() {
							textures.push(None);
						}
						textures[n] = Some(tex_creator.create_texture_from_surface(&surf)?);
					}
				}
			}
		}
	}
	if textures.len() < NUM_BLOCKS {
		return GenError::new(format!("failed to load {} textures from {}", NUM_BLOCKS, &tex_dir));
	}

	let mut disp = Display::new(canvas, &textures);
	let (w, h) = disp.size();

	// load map from command line
	let args: Vec<String> = env::args().skip(1).collect();
	let mut editor = match args.len() {
		0 => Editor::new(w, h, find_map1(&PathBuf::from(map_dir))?, false),
		1 => Editor::new(w, h, PathBuf::from(&args[0]), true),
		_ => return GenError::new(format!("need 0 or 1 arguments, have {:?}", args)),
	};

	// Main loop
	let mut key_states = KeyStates::new();
	let mut key_debounce = KeyStates::new();
	let mut start = time::Instant::now();
	loop {
		// Advance time
		editor.tick(key_states.merge(key_debounce));
		{
			let el = start.elapsed().as_millis();
			if el > 32 {
				println!("MISSED FRAME after {}ms, catching up", el);
				editor.tick(key_states.merge(key_debounce));
			}
			if el > 48 {
				println!("Degrading from 30 to 20 FPS");
				editor.tick(key_states.merge(key_debounce));
			}
			start = time::Instant::now();
		}

		// Render
		editor.render(&mut disp);
		disp.present();

		// Event handling
		key_debounce.clear();
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } => return Ok(()),
				Event::MouseMotion { x, y, mousestate, .. } => {
					editor.handle_mouse(Pt(x, y), mousestate.left(), mousestate.right());
				}
				Event::MouseButtonDown { x, y, mouse_btn, .. } => {
					editor.handle_mouse(
						Pt(x, y),
						mouse_btn == mouse::MouseButton::Left,
						mouse_btn == mouse::MouseButton::Right,
					);
				}
				Event::MouseWheel { x, y, .. } => editor.handle_mouse_wheel(x, y),
				Event::KeyDown { keycode, .. } => {
					if let Some(keycode) = keycode {
						let k = keymap(keycode);
						key_debounce.set_down(k, true);
						key_states.set_down(k, true);
						editor.handle_key(k);
					}
				}
				Event::KeyUp { keycode, .. } => {
					if let Some(keycode) = keycode {
						let k = keymap(keycode);
						key_states.set_down(k, false);
					}
				}
				Event::Window { win_event, .. } => match win_event {
					event::WindowEvent::Resized(x, y) => editor.handle_resize(x, y),
					event::WindowEvent::SizeChanged(x, y) => editor.handle_resize(x, y),
					_ => (),
				},
				_ => (),
			}
		}
	}
}

fn keymap(sdl_key: sdl2::keyboard::Keycode) -> Key {
	use sdl2::keyboard::Keycode;
	match sdl_key {
		Keycode::Left => Key::Left,
		Keycode::S => Key::Left,
		Keycode::Right => Key::Right,
		Keycode::F => Key::Right,
		Keycode::Up => Key::Up,
		Keycode::E => Key::Up,
		Keycode::Down => Key::Down,
		Keycode::D => Key::Down,
		Keycode::Space => Key::A,
		Keycode::LAlt => Key::B,
		Keycode::RAlt => Key::B,
		Keycode::Equals => Key::ZoomIn,
		Keycode::Minus => Key::ZoomOut,
		Keycode::P => Key::Pause,
		Keycode::W => Key::Save,
		Keycode::N => Key::NextMap,
		Keycode::M => Key::PrevMap,
		Keycode::R => Key::Restart,
		_ => Key::None,
	}
}

fn init_window() -> Result<(Canvas, EventPump)> {
	let context = sdl2::init()?;
	let video = context.video()?;
	let window = video.window("game", 1920 / 2, 1080 / 2).resizable().position_centered().build()?;
	match window.into_canvas().present_vsync().build() {
		Ok(c) => Ok((c, context.event_pump()?)),
		Err(e) => Err(Box::new(e)),
	}
}
