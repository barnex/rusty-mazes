use crate::prelude::*;

/// A half-open rectangle
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Rect {
	pub min: Pt, // top-left vertex, considered inside
	pub max: Pt, // bottom-right vertex, considered outside
}

impl Rect {
	/// Construct a semi-open rectangle with given top-left vertex ("origin"),
	/// and width and height (>= 0).
	pub fn new(topleft: Pt, width: i32, height: i32) -> Rect {
		assert!(width >= 0);
		assert!(height >= 0);
		Rect {
			min: topleft,
			max: topleft + Pt(width, height),
		}
	}

	/// The 4 vertices that are fully inside the half-open rectangle.
	/// I.e., offset by 1 at the bottom, right edge to be inside.
	pub fn vertices_incl(&self) -> [Pt; 4] {
		[
			Pt(self.min.x() - 0, self.min.y() - 0),
			Pt(self.max.x() - 1, self.min.y() - 0),
			Pt(self.max.x() - 1, self.max.y() - 1),
			Pt(self.min.x() - 0, self.max.y() - 1),
		]
	}

	/// Test if p lies inside the semi-open rectangle.
	///    
	///     use game::prelude::*;
	///     let r = Rect::new(Pt(1,2),3,4);
	///     assert!(!r.inside(Pt(0, 2)));
	///     assert!(!r.inside(Pt(1, 1)));
	///     assert!( r.inside(Pt(1, 2)));
	///     assert!( r.inside(Pt(3, 2)));
	///     assert!(!r.inside(Pt(4, 2))); // it's half-open!
	///     assert!( r.inside(Pt(1, 5)));
	///     assert!(!r.inside(Pt(1, 6))); // it's half open!
	///
	pub fn inside(&self, p: Pt) -> bool {
		p.0 >= self.min.0 && p.0 < self.max.0 && p.1 >= self.min.1 && p.1 < self.max.1
	}

	/// Test if two semi-open rectangles overlap (at least partially).
	///    
	///     use game::prelude::*;
	///     let r = Rect::new(Pt(0, 0),10, 10);
	///     assert!(r.overlaps(Rect::new(Pt(0, 0), 1, 1)));
	///     assert!(r.overlaps(Rect::new(Pt(0, 0), 10, 10)));
	///     assert!(r.overlaps(Rect::new(Pt(0, 0), 20, 20)));
	///     assert!(r.overlaps(Rect::new(Pt(1, 1), 1, 1)));
	///     assert!(r.overlaps(Rect::new(Pt(1, 1), 20, 20)));
	///     assert!(r.overlaps(Rect::new(Pt(9, 9), 20, 20)));
	///     assert!(!r.overlaps(Rect::new(Pt(9, 10), 20, 20)));
	///     assert!(!r.overlaps(Rect::new(Pt(10, 9), 20, 20)));
	///     assert!(!r.overlaps(Rect::new(Pt(10, 10), 20, 20)));
	///
	pub fn overlaps(self, b: &Rect) -> bool {
		interv_overlap((self.min.0, self.max.0), (b.min.0, b.max.0)) && interv_overlap((self.min.1, self.max.1), (b.min.1, b.max.1))
	}
}

// do half-open intervals a and b (at least partially) overlap?
#[inline]
fn interv_overlap(a: (i32, i32), b: (i32, i32)) -> bool {
	inside_interv(a.0, b) || inside_interv(b.0, a)
}

// does x lie in the half-open interval i?
#[inline]
fn inside_interv(x: i32, i: (i32, i32)) -> bool {
	x >= i.0 && x < i.1
}
