use crate::prelude::*;

use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Load a map in JSON format.
pub fn load(p: &Path) -> Result<Map> {
	println!("loading {}", p.to_string_lossy());
	let f = File::open(p)?;
	let b = io::BufReader::new(f);
	let map = serde_json::from_reader(b)?;
	Ok(map)
}

/// Save a map in JSON format.
pub fn save(map: &Map, p: &Path) -> Result<()> {
	let f = File::create(p)?;
	let mut b = io::BufWriter::new(f);
	serde_json::to_writer(&mut b, map)?;
	b.flush()?;
	println!("wrote {}", p.to_string_lossy());
	Ok(())
}

/// Separate a map's blocks into static blocks (Map) and Movers.
pub fn unstage(staging: &Map) -> (Map, Vec<Mover>) {
	let mut map = Map::new();
	let mut movers = vec![Mover::new(Pt(1, 1) * GRID, PLAYER)];

	for (iy, row) in staging.blocks.iter().enumerate() {
		for (ix, blk) in row.iter().enumerate() {
			let grid = Pt(ix as i32, iy as i32);

			if let Some(mover) = Mover::unstage(grid * GRID, *blk) {
				// player is special: there is exactly one, and it's the first mover
				if mover.typ() == PLAYER {
					movers[0].pos = mover.pos;
				} else {
					movers.push(mover)
				}
			} else {
				map.set(grid, *blk);
			}
		}
	}
	(map, movers)
}

/// Given a map file, find the next map (alphabetically) in the same directory.
/// Find the previous map if delta = -1.
pub fn find_next_map(curr: &Path, delta: i32) -> Result<PathBuf> {
	let parent = match curr.parent() {
		None => return GenError::new(format!("find_next_map: file '{}' has not parent directory", curr.to_string_lossy())),
		Some(p) => p,
	};
	let ls = ls_maps(parent)?;
	if let Some(i) = ls.iter().position(|x| x == curr) {
		let mut i = (i as i32) + delta;
		if i < 0 {
			i = (ls.len() as i32) - 1;
		}
		if i >= (ls.len() as i32) {
			i = 0;
		};
		Ok(ls[i as usize].clone())
	} else {
		GenError::new(format!(
			"find_next_map: current map {} not found in {}",
			curr.to_string_lossy(),
			parent.to_string_lossy()
		))
	}
}

/// Find the first map (alphabetically) in a directory.
pub fn find_map1(dir: &Path) -> Result<PathBuf> {
	let ls = ls_maps(dir)?;
	match ls.len() {
		0 => GenError::new(format!("no map found in '{}'", dir.to_string_lossy())),
		_ => Ok(ls[0].clone()),
	}
}

/// List all maps (json files) in directory dir, alphabetically.
fn ls_maps(dir: &Path) -> Result<Vec<PathBuf>> {
	let mut ls: Vec<PathBuf> = fs::read_dir(dir)?
		.map(|x| x.unwrap().path())
		.filter(|x| x.extension() == Some(&OsStr::new("json")))
		.collect();
	ls.sort();
	Ok(ls)
}
