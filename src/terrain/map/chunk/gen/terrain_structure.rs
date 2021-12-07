
use vek::Vec2;
use rand::Rng;

pub trait Structure {
	fn gen_block(&mut self, x: i32, y: i32) -> (Option<usize>, Option<usize>);
	fn position(&self) -> Vec2<i32>;
	fn width(&self) -> i32;
	fn height(&self) -> i32;
}

pub struct Vein {
	vtype: usize,
	wtype: Option<usize>,
	pos: Vec2<i32>,
	width: i32,
	height: i32,
}

impl Vein {
	pub fn new(
			pos: Vec2<i32>, block_type: usize,
			wall_type: Option<usize>, width: i32, height: i32
		) -> Vein {
		Vein {
			vtype: block_type,
			wtype: wall_type,
			pos,
			width,
			height,
		}
	}
}

impl Structure for Vein {
	fn gen_block(&mut self, _x: i32, _y: i32) -> (Option<usize>, Option<usize>) {
		let mut rng = rand::thread_rng();
		return (Some(self.vtype), self.wtype);
		if rng.gen_bool(0.7) {
			(Some(self.vtype), self.wtype)
		} else {
			(None, None)
		}
	}
	fn position(&self) -> Vec2<i32> {
		self.pos
	}
	fn width(&self) -> i32 {
		self.width
	}
	fn height(&self) -> i32 {
		self.height
	}
}
