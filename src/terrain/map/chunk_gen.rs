use vek::Vec2;
use super::{
	ChunkContent,
	super::blocks::{self}
};


pub fn build(chunk_pos: &Vec2<i32>) -> ChunkContent {
	if chunk_pos.y == -1 && (chunk_pos.x == -1 || chunk_pos.x == 0) {
		ChunkContent::full_block(0, 1)
	} else {
		if chunk_pos.x == 0 && chunk_pos.y == 0 {
			return ChunkContent::full_block(2, 2)
		}
		let _i = ((chunk_pos.x + chunk_pos.y + 10*blocks::NB_BLOCK_TYPES as i32) % blocks::NB_BLOCK_TYPES as i32).abs() as usize;
		ChunkContent::full_block(1, 1)
	}
}