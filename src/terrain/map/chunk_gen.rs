use vek::Vec2;
use super::{
	ChunkContent
};


pub fn build(chunk_pos: &Vec2<i32>) -> ChunkContent {
	if chunk_pos.y == -1 && (chunk_pos.x == -1 || chunk_pos.x == 0) {
		ChunkContent::full_block(0, 1)
	} else {
		//let _i = ((chunk_pos.x + chunk_pos.y + 10*blocks::NB_BLOCK_TYPES as i32) % blocks::NB_BLOCK_TYPES as i32).abs() as usize;
		ChunkContent::random(&[
			(1,1,50),
			(2,1,10),
			(2,2,20),
			(0,1,1),
			(3,1,2),
			(4,1,2),
		])
	}
}