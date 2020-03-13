#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Key {
	None = 0,
	Left = 1,
	Right = 2,
	Up = 3,
	Down = 4,
	A = 5,
	B = 6,
	X = 7,
	Pause = 8,
	ZoomIn = 9,
	ZoomOut = 10,
	Save = 11,
	PrevMap = 12,
	Restart = 13,
	NextMap = 14,
}

impl Key {
	#[inline]
	pub fn id(self) -> usize {
		self as usize
	}
}

// KeyStates records which of the lowest 8 keys are currently pressed down.
// these are the "dynamic" keys (movement, jumping) that need to be timed precisely
// and indepdently of the OS key repeat rate.
#[derive(Copy, Clone, Debug)]
pub struct KeyStates {
	pub down: [bool; 8],
}

impl KeyStates {
	pub fn new() -> KeyStates {
		KeyStates { down: [false; 8] }
	}

	pub fn set_down(&mut self, k: Key, down: bool) {
		let k = k.id();
		if k < self.down.len() {
			self.down[k] = down;
		}
	}

	#[must_use]
	pub fn merge(self, b: KeyStates) -> KeyStates {
		let mut down = [false; 8];
		for i in 0..down.len() {
			down[i] = self.down[i] || b.down[i];
		}
		KeyStates { down }
	}

	pub fn clear(&mut self) {
		self.down = [false; 8];
	}
}
