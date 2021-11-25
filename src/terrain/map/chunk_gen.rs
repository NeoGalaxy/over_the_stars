use super::{
	ChunkContent,
	super::blocks::{self}
};


pub fn build(chunk_pos: &[i32]) -> ChunkContent {
	if chunk_pos[1] == -1 && (chunk_pos[0] == -1 || chunk_pos[0] == 0) {
		ChunkContent::full_block(0, 1)
	} else {
		if chunk_pos[0] == 0 && chunk_pos[1] == 0 {
			return ChunkContent::full_block(2, 2)
		}
		let _i = ((chunk_pos[0] + chunk_pos[1] + 10*blocks::NB_BLOCK_TYPES as i32) % blocks::NB_BLOCK_TYPES as i32).abs() as usize;
		ChunkContent::full_block(1, 1)
	}
}