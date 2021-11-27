extern crate sdl2;

use sdl2::rect::Rect;

use sdl2::render::{Canvas,RenderTarget};
use sdl2::pixels::Color;
use std::cmp;
mod types;
pub use types::BlockType;
pub const BLOCK_SIZE: u32 = 16;

#[derive(Debug, Copy, Clone)]
pub struct Block {
	pub block : &'static BlockType,
	pub wall : &'static BlockType,
	pub light : i32,
	pub entity_light : i32,
	pub entity_light_frame : i64,
	pub break_progress : u32
}

impl Default for Block {
	fn default() -> Block {
		Block {
			block: types::get(0),
			wall: types::get(0),
			light: 0,
			entity_light: 0,
			entity_light_frame: -1,
			break_progress: 0
		}
	}
}

pub const NB_BLOCK_TYPES: usize = types::BLOCK_TYPES.len();

impl Block {
	pub fn from_id(id: usize, bg_id: usize) -> Block {
		let block_type = types::get(id);
		let wall_type = types::get(bg_id);
		Block {
			block: block_type,
			wall: wall_type,
			light: cmp::max(0, block_type.light_level),
			entity_light: 0,
			entity_light_frame: -1,
			break_progress: 0
		}
	}
	pub fn set_block(&mut self, id: usize) {
		self.block = types::get(id);
	}
	pub fn intern_light(&self) -> i32 {
		return self.block.light_level;
	}
	pub fn disp<T: RenderTarget>(&self, canvas : &mut Canvas<T>, x : i32, y : i32) {
		let block_rect = Rect::new(x, y, BLOCK_SIZE, BLOCK_SIZE);
		if let Some(color) = self.block.texture {
			canvas.set_draw_color(color);
			canvas.fill_rect(block_rect).unwrap();
			match self.block.outline_texture {
				Some(color) => canvas.set_draw_color(color),
				None => (),
			}
			canvas.draw_rect(block_rect).unwrap();
			canvas.draw_rect(Rect::new(x+1, y+1, BLOCK_SIZE-2, BLOCK_SIZE-2)).unwrap();
			canvas.draw_rect(Rect::new(x+2, y+2, BLOCK_SIZE-4, BLOCK_SIZE-4)).unwrap();
			canvas.draw_rect(Rect::new(x+3, y+3, BLOCK_SIZE-6, BLOCK_SIZE-6)).unwrap();
		} else if let Some(color) = self.wall.bg_texture {
			canvas.set_draw_color(color);
			canvas.fill_rect(block_rect).unwrap();
		}
		if self.light < 37 {
			if self.light <= 0 {
				canvas.set_draw_color(Color::BLACK);
			} else {
				canvas.set_draw_color(Color::RGBA(0, 0, 0, 255 - (255 * (self.light + 3) / 40) as u8));
			}
			canvas.fill_rect(block_rect).unwrap();
		}
	}
	pub fn update_entity_light(&mut self, light: i32, frame: i64) -> bool {
		if frame > self.entity_light_frame || light > self.entity_light {
			self.entity_light_frame = frame;
			self.entity_light = light;
			return true;
		}
		return false;
	}
}
