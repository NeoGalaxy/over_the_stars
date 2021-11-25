extern crate sdl2;

use sdl2::rect::Rect;

use sdl2::render::{Canvas,RenderTarget};
use std::cmp;
mod types;
pub use types::BlockType;
pub const BLOCK_SIZE: u32 = 16;

#[derive(Debug, Copy, Clone)]
pub struct Block {
	pub block : &'static BlockType,
	pub wall : &'static BlockType,
	pub light : i32,
	pub break_progress : u32
}

impl Default for Block {
	fn default() -> Block {
		Block::from_id(1, 1)
	}
}

pub const NB_BLOCK_TYPES: usize = types::BLOCK_TYPES.len();

impl Block {
	pub fn from_id(id: usize, bg_id: usize) -> Block {
		let block_type = types::get(id);
		let wall_type = types::get(bg_id);
		Block {
			block : block_type,
			wall : wall_type,
			light : cmp::max(0, block_type.light_level),
			break_progress : 0
		}
	}
	pub fn set_block(&mut self, id: usize) {
		self.block = types::get(id);
	}
	pub fn disp<T: RenderTarget>(&self, canvas : &mut Canvas<T>, x : i32, y : i32) {
		if let Some(color) = self.block.texture {
			canvas.set_draw_color(color);
			let block_rect = Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE);
			canvas.fill_rect(block_rect).unwrap();
			match self.block.outline_texture {
				Some(color) => canvas.set_draw_color(color),
				None => (),
			}
			canvas.draw_rect(block_rect).unwrap();
		} else if let Some(color) = self.wall.bg_texture {
			canvas.set_draw_color(color);
			let block_rect = Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE);
			canvas.fill_rect(block_rect).unwrap();
		}
	}
}
