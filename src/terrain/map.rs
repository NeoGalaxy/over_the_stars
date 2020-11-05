extern crate sdl2;
extern crate vek;

mod chunk_gen;

//use std::io::Write;
use std::{
	collections::HashMap,
	vec::Vec,
	boxed::Box,
};
use rand::Rng;
use sdl2::render::{Canvas, RenderTarget};
use vek::{vec::repr_c::vec2::Vec2/*, geom::repr_c::Rect as vRect*/};
use super::blocks::{Block, BLOCK_SIZE};
use crate::entities::{
	Entity,
	Task,
	//Player
};

// use super::blocks::walls;


const CHUNK_SIZE : u32 = 16;

pub struct ChunkContent<T> {
	blocks: [[Block; CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
	entities: HashMap<u64, Box<dyn Entity<T>>>,
}

//#[derive(Debug)]
impl<T: RenderTarget> ChunkContent<T> {
	fn full_block(id: usize) -> ChunkContent<T> {
		ChunkContent{
			blocks : [[Block::from_id(id); CHUNK_SIZE as usize]; CHUNK_SIZE as usize],
			entities : HashMap::new(),
		}
	}
	fn add_entity<E: 'static +  Entity<T>>(&mut self, entity: E, uid: u64) {
		self.entities.insert(uid, Box::new(entity));
	}
}

//#[derive(Debug)]
pub struct Map<T> {
	chunks : HashMap<Vec2<i32>,ChunkContent<T>>,
	inactive : HashMap<Vec2<i32>,ChunkContent<T>>,
	entities : HashMap<u64, Vec2<i32>>,
	players : HashMap<u64, Vec2<i32>>,
}

impl<T: RenderTarget> Map<T> {
	pub fn new() -> Map<T> {
		Map {
			chunks : HashMap::new(),
			inactive : HashMap::new(),
			entities : HashMap::new(),
			players : HashMap::new(),
		}
	}
	pub fn add_entity<E: 'static +  Entity<T>>(&mut self, entity: E) -> u64 {
		let mut rng = rand::thread_rng();
		let uid = loop {
			let uid = rng.gen::<u64>();
			if !self.entities.contains_key(&uid) {
				break uid;
			}
		};
		//entity.set_uid(uid);
		let pos = entity.get_pos();
		let chunk_pos: Vec2<i32> = (pos / CHUNK_SIZE as f64).floor().as_();
		self.get_mut_ch(chunk_pos).add_entity(entity, uid);
		self.entities.insert(uid, chunk_pos);
		uid
	}
	fn get_ch(&mut self, pos: Vec2<i32>) -> &ChunkContent<T> {
		if !self.chunks.contains_key(&pos) {
			self.chunks.insert(pos, chunk_gen::build(&pos));
		}
		&self.chunks[&pos]
	}
	fn get_active_ch(&self, pos: Vec2<i32>) -> Option<&ChunkContent<T>> {
		self.chunks.get(&pos)
	}
	fn get_mut_ch(&mut self, pos: Vec2<i32>) -> &mut ChunkContent<T> {
		if !self.chunks.contains_key(&pos) {
			self.chunks.insert(pos, chunk_gen::build(&pos));
		}
		self.chunks.get_mut(&pos).unwrap()
	}
	fn set_ch_block(&mut self, chunk_pos: &Vec2<i32>, block_pos: &Vec2<usize>, block : Block) {
		self.chunks.get_mut(chunk_pos).unwrap().blocks[block_pos.x][block_pos.y] = block;
	}
	fn set_block(&mut self, pos: &Vec2<i32>, block_id : usize) {
		let chunk_pos : Vec2<i32> = pos / Vec2::broadcast(CHUNK_SIZE as i32);
		let block_pos = {
			let mut res = pos - chunk_pos * Vec2::broadcast(CHUNK_SIZE as i32);
			if pos.x < 0 && chunk_pos.x * CHUNK_SIZE as i32 != pos.x {
				res.x += CHUNK_SIZE as i32;
			}
			if pos.y < 0 && chunk_pos.y * CHUNK_SIZE as i32 != pos.y {
				res.y += CHUNK_SIZE as i32;
			}
			res
		};
		let block_pos : Vec2<usize> = block_pos.as_();
		self.set_ch_block(&chunk_pos, &block_pos, Block::from_id(block_id))
	}
	pub fn get_block(&mut self, pos: Vec2<i32>) -> &Block {
		let chunk_pos : Vec2<i32> = pos / Vec2::broadcast(CHUNK_SIZE as i32);
		let block_pos = {
			let mut res = pos - chunk_pos * Vec2::broadcast(CHUNK_SIZE as i32);
			if pos.x < 0 && chunk_pos.x * CHUNK_SIZE as i32 != pos.x {
				res.x += CHUNK_SIZE as i32;
			}
			if pos.y < 0 && chunk_pos.y * CHUNK_SIZE as i32 != pos.y {
				res.y += CHUNK_SIZE as i32;
			}
			res
		};
		if block_pos.x < 0 || block_pos.x >= BLOCK_SIZE as i32
		|| block_pos.y < 0 || block_pos.y >= BLOCK_SIZE as i32 {
			println!("-------------------------------------------");
			println!("P {:?}", pos);
			println!("C {:?}", chunk_pos * Vec2::broadcast(CHUNK_SIZE as i32));
			println!("B {:?}", block_pos);
			panic!("Invalid index !!! Contact developper and give him the aove values.");
		}
		let block_pos : Vec2<usize> = block_pos.as_();
		&self.get_ch(chunk_pos).blocks[block_pos.x][block_pos.y]
	}
	pub fn disp(&mut self, canvas : &mut Canvas<T>, camera : &Vec2<f64>) {
		let (width, height) = canvas.output_size().unwrap();
		let start : Vec2<i32> = ((camera.floor() - camera) * BLOCK_SIZE as f64).as_();
		let block_start : Vec2<i32> = (camera.floor() - 
			(Vec2::new(width as f64, height as f64) / BLOCK_SIZE as f64 / 2_f64)).as_();
		for (px_x, x) in (start.x..(width + 1) as i32).step_by(BLOCK_SIZE as usize)
		             .zip(block_start.x..block_start.x + (width/BLOCK_SIZE + 2) as i32) {
			for (px_y, y) in (start.y..(height + 1) as i32).step_by(BLOCK_SIZE as usize)
			             .zip(block_start.y..block_start.y + (height/BLOCK_SIZE + 2) as i32) {
				self.get_block(Vec2::new(x, y)).disp(canvas, px_x, px_y);
			}
		}

		for (uid,chunk_coord) in self.entities.iter() {
			self.get_active_ch(*chunk_coord).unwrap().entities[&uid].disp(canvas, camera);
		}
	}
	pub fn place(&mut self, id: usize, pos: Vec2<i32>) {
		let chunk_pos : Vec2<i32> = pos / Vec2::broadcast(CHUNK_SIZE as i32);
		let block_pos: Vec2<usize> = {
			let pos: Vec2<usize> = pos.as_(); let chunk_pos: Vec2<usize> = chunk_pos.as_();
			pos - chunk_pos * Vec2::broadcast(CHUNK_SIZE as usize)
		};
		self.set_ch_block(&chunk_pos, &block_pos, Block::from_id(id));
	}
	pub fn update_active(&mut self) {
		let mut active_entities: Vec<(u64,Vec2<i32>)> = Vec::new();
		for tmp in self.entities.iter() {
			active_entities.push((*tmp.0, *tmp.1))
		}
		let mut all_tasks = Vec::new();
		for (uid, chunk_pos) in active_entities {
			all_tasks.push((
				self.get_active_ch(chunk_pos).unwrap().entities[&uid].update(&self),
				uid,
				chunk_pos
			));
		}
		for (entity_tasks, uid, chunk_pos) in all_tasks {
			for task in entity_tasks {
				match task {
					Task::Move(pos) => {
						self.get_mut_ch(chunk_pos).entities.get_mut(&uid).unwrap().r#move(pos);
					},
					Task::Break(pos) => {
						self.set_block(&pos, 0);
					},
					Task::Place(pos, id) => {
						self.set_block(&pos, id);
					},
				}
			}
		}
	}

}
