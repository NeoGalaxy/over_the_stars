extern crate sdl2;

use sdl2::render::{Canvas,RenderTarget};
use std::collections::HashMap;
use super::blocks::{Block, BLOCK_SIZE};

// use super::blocks::walls;


const CHUNK_SIZE : u32 = 16;
#[derive(Debug)]
pub struct ChunkMap {
	chunks : HashMap<(i32,i32),ChunkContent>
}

impl ChunkMap {
	pub fn new() -> ChunkMap {
		ChunkMap {
			chunks : HashMap::new()
		}
	}
	pub fn get(&mut self, chunk_x : i32, chunk_y : i32) -> Chunk {
		if !self.chunks.contains_key(&(chunk_x, chunk_y)) {
			self.chunks.insert((chunk_x, chunk_y), ChunkContent::new());
		}
		Chunk{chunk_x, chunk_y, map : &mut *self}
	}
}

#[derive(Debug)]
struct ChunkContent {
	blocks: [[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
	// walls: [[&Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
	entities: Vec<i32>,
}

impl ChunkContent {
	fn new() -> ChunkContent {
		ChunkContent{
			blocks : Default::default(),
			entities : Vec::new()
		}
	}
}

#[derive(Debug)]
pub struct Chunk<'a> {
	chunk_x : i32,
	chunk_y : i32,
	map : &'a mut ChunkMap
}

impl<'a> Chunk<'a> {
	pub fn get_block<'b>(&'b self, block_x: usize, block_y: usize) -> &'b Block {
		&self.map.chunks[&(self.chunk_x,self.chunk_y)].blocks[block_x][block_y]
	}
	pub fn disp<T: RenderTarget>(&self, canvas : &mut Canvas<T>, x : i32, y : i32) {
		let content = &self.map.chunks[&(self.chunk_x,self.chunk_y)];
		let step = (BLOCK_SIZE-1)  as usize;
		let x_start = x as i32 - (CHUNK_SIZE * BLOCK_SIZE-1) as i32 / 2;
		let x_end = x_start as i32 + (CHUNK_SIZE * BLOCK_SIZE-1) as i32;
		let y_start = y as i32 - (CHUNK_SIZE * BLOCK_SIZE-1) as i32 / 2;
		let y_end = y_start as i32 + (CHUNK_SIZE * BLOCK_SIZE-1) as i32;

		for (block_pos_x, block_col) in (x_start..x_end).step_by(step).zip(content.blocks.iter()) {
			for (block_pos_y, block) in (y_start..y_end).step_by(step).zip(block_col.iter()) {
				block.disp(canvas, block_pos_x, block_pos_y)
			}
		}
	}
	pub fn place(&mut self, id: usize, x: usize, y: usize) {
		let mut content = &mut self.map.chunks.get_mut(&(self.chunk_x,self.chunk_y)).unwrap();
		content.blocks[x][y] = Block::from_id(id);
	}
}
