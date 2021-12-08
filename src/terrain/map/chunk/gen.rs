pub mod terrain_structure;

use std::cmp::max;
use std::cmp::min;
use super::super::CHUNK_SIZE;
use std::collections::HashMap;
use terrain_structure::Structure;
use vek::Vec2;
use super::{
	ChunkContent
};
use rand::Rng;
use terrain_structure::Vein;

pub struct TerrainGenerator {
	chunks_structures: HashMap<Vec2<i32>, Vec<Box<dyn Structure>>>
}

impl TerrainGenerator {
	pub fn new() -> TerrainGenerator {
		TerrainGenerator {
			chunks_structures: HashMap::new()
		}
	}
	pub fn build(&mut self, chunk_pos: &Vec2<i32>) -> ChunkContent {
		/*if chunk_pos.y == 0 && (chunk_pos.x == -1 || chunk_pos.x == 0) {
			return ChunkContent::full_block(0, 1);
		}*/
		let mut ret = ChunkContent::full_block(1, 1);
		for chunk_y in (chunk_pos.y - 1)..=(chunk_pos.y + 1) {
			for chunk_x in (chunk_pos.x - 1)..=(chunk_pos.x + 1) {
				let neighbor_chunk_pos = Vec2::new(chunk_x, chunk_y);
				self.gen_struct(&neighbor_chunk_pos);
				//for (chunk, ch_structures) in self.chunks_structures.iter_mut() {
					let chunk_st = self.chunks_structures.get_mut(&neighbor_chunk_pos).unwrap();
					for structure in chunk_st.iter_mut() {
						let pos = structure.position();
						let width = structure.width();
						let height = structure.height();
						let ystart = max(pos.y - height/2, chunk_pos.y * CHUNK_SIZE as i32);
						let yend = min(pos.y + height/2, (chunk_pos.y + 1) * CHUNK_SIZE as i32);
						let xstart = max(pos.x - width/2, chunk_pos.x * CHUNK_SIZE as i32);
						let xend = min(pos.x + width/2, (chunk_pos.x + 1) * CHUNK_SIZE as i32);
						for block_y in ystart..yend {
							let ch_block_y = block_y - chunk_pos.y * CHUNK_SIZE as i32;
							for block_x in xstart..xend {
								let ch_block_x = block_x - chunk_pos.x * CHUNK_SIZE as i32;
								let block_types = structure.gen_block(block_x, block_y);
								if let Some(block) = block_types.0 {
									ret.blocks[ch_block_x as usize][ch_block_y as usize].set_block(block);
								}
								if let Some(wall) = block_types.1 {
									ret.blocks[ch_block_x as usize][ch_block_y as usize].set_wall(wall);
								}
							}
						}
					}
				//}
			}
		}
		return ret;
	}
	pub fn gen_struct(&mut self, chunk_pos: &Vec2<i32>) {
		if self.chunks_structures.contains_key(chunk_pos) {
			return;
		}
		let mut rng = rand::thread_rng();
		let nb_st = rng.gen_range(1, 2);
		let mut st: Vec<Box<dyn Structure>> = Vec::with_capacity(nb_st);
		for _ in 0..nb_st {
			let x = rng.gen_range(0, CHUNK_SIZE as i32);
			let y = rng.gen_range(0, CHUNK_SIZE as i32);
			let b_type = rng.gen_range(3, 6);
			let height = rng.gen_range(2, 5);
			let width = rng.gen_range(3, 9);
			st.push(Box::new(Vein::new(Vec2::new(x,y) + chunk_pos * CHUNK_SIZE as i32, b_type, Some(1), width, height)));
		}
		self.chunks_structures.insert(*chunk_pos, st);
	}
}

