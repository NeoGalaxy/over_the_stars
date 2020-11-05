use sdl2::render::RenderTarget;
use vek::vec::repr_c::vec2::Vec2;

use super::{
	ChunkContent,
	super::blocks::{self}
};


pub fn build<T: RenderTarget>(chunk_pos: &Vec2<i32>) -> ChunkContent<T> {
	let i = ((chunk_pos.x + chunk_pos.y) % blocks::NB_BLOCK_TYPES as i32).abs() as usize;
	ChunkContent::full_block(i)
}