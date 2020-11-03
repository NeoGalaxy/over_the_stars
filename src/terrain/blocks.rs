extern crate sdl2;


use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::{Canvas,RenderTarget};
use std::cmp;
mod types;
use types::{BlockType};
pub const BLOCK_SIZE: u32 = 16;

#[derive(Debug)]
pub struct Block {
	block : &'static BlockType,
	wall : &'static BlockType,
	light : i32,
	break_progress : u32
}

impl Default for Block {
	fn default() -> Block {
		Block::from_id(1)
	}
}

impl Block {
	/*fn new(block_type: &'static BlockType) -> Block {
		Block {
			block : block_type,
			wall : block_type,
			light : cmp::max(0, block_type.light_level),
			break_progress : 0
		}
	}*/
	pub fn from_id(id: usize) -> Block {
		let block_type = types::get(id);
		Block {
			block : block_type,
			wall : block_type,
			light : cmp::max(0, block_type.light_level),
			break_progress : 0
		}
	}
	pub fn disp<T: RenderTarget>(&self, canvas : &mut Canvas<T>, x : i32, y : i32) {
		match self.block.texture {
			Some(color) => canvas.set_draw_color(color),
			None => canvas.set_draw_color(Color::RGB(0,0,0)),
		}
		let block_rect = Rect::from_center(Point::new(x,y), BLOCK_SIZE, BLOCK_SIZE);
		canvas.fill_rect(block_rect).unwrap();
		canvas.set_draw_color(Color::RGB(0,0,0));
		canvas.draw_rect(block_rect).unwrap();
	}
}
