use std::collections::HashMap;
use sdl2::render::Canvas;
use sdl2::video::Window;
use vek::Vec2;

pub const CHUNK_SIZE : u32 = 16;

use crate::entities::Entity;
use super::{
	Block,
	BLOCK_SIZE
};

//#[derive(Debug)]
pub struct ChunkContent {
	pub blocks: [[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
	pub entities: HashMap<u64, Box<dyn Entity>>,
}

impl ChunkContent {
	pub fn full_block(id: usize) -> ChunkContent {
		ChunkContent{
			blocks : [[Block::from_id(id); CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
			entities : HashMap::new(),
		}
	}
	pub fn add_entity(&mut self, entity: Box<dyn Entity>, uid: u64) {
		self.entities.insert(uid, entity);
	}
	pub fn disp(&self, canvas : &mut Canvas<Window>, pos: Vec2<i32>, camera : &Vec2<f64>, level: i8) {
		match level {
			0 => {
				let (width, height) = canvas.output_size().unwrap();
				let top_left_px = {
					let screen_center = Vec2::new(width as i32, height as i32)/2;
					let absolute_pos_px = pos * CHUNK_SIZE as i32 * BLOCK_SIZE as i32;
					let px_camera = camera * BLOCK_SIZE as f64;
					screen_center + absolute_pos_px - px_camera.as_()
				};
				for x in 0..CHUNK_SIZE {
					for y in 0..CHUNK_SIZE {
						let block_in_chunk_pos_px = Vec2::new(x*BLOCK_SIZE, y*BLOCK_SIZE).as_();
						let block_pos_px = top_left_px + block_in_chunk_pos_px;
						self.blocks[x as usize][y as usize].disp(canvas, block_pos_px.x, block_pos_px.y);
					}
				}
			},
			1 => {
				for (_,entity) in self.entities.iter() {
					entity.disp(canvas, camera)
				}
			},
			_ => unreachable!()
		};
	}
}
