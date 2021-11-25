extern crate sdl2;
extern crate vek;

pub mod chunk_gen;
pub mod chunk;

use chunk::{
	ChunkContent,
	CHUNK_SIZE
};
use sdl2::video::Window;

use std::{
	collections::HashMap,
	vec::Vec,
	boxed::Box,
	time::{Instant},
};
use rand::Rng;
use sdl2::render::{Canvas};
use vek::{vec::repr_c::vec2::Vec2/*, geom::repr_c::Rect as vRect*/};
use super::blocks::{Block, BLOCK_SIZE};
use crate::entities::{
	Entity,
	Action,
	Aiming,
	Task,
	//Player
};

// use super::blocks::walls;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interaction {
	Up,
	Down,
	Left,
	Right,
	AimAt(Vec2<i32>),
	Attack,
	Use
}

//#[derive(Debug)]
pub struct Map {
	chunks: HashMap<Vec2<i32>,ChunkContent>,
	//inactive: HashMap<Vec2<i32>,ChunkContent>,
	entities: HashMap<u64, Vec2<i32>>,
	//players: HashMap<u64, Vec2<i32>>,
	last_update: Instant,
	interaction_subject_uid: u64,
}

impl Map {
	pub fn new() -> Map {
		Map {
			chunks : HashMap::new(),
			//inactive : HashMap::new(),
			entities : HashMap::new(),
			//players : HashMap::new(),
			last_update : Instant::now(),
			interaction_subject_uid : 0,
		}
	}
	pub fn add_entity(&mut self, entity: Box<dyn Entity>) -> u64 {
		let mut rng = rand::thread_rng();
		let uid = loop {
			let uid: u64 = rng.gen();
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

	pub fn see_entity(&mut self, uid: u64) -> &Box<dyn Entity> {
		&self.get_active_ch(self.entities[&uid]).unwrap().entities[&uid]
	}

	fn get_ch(&mut self, pos: Vec2<i32>) -> &ChunkContent {
		if !self.chunks.contains_key(&pos) {
			self.chunks.insert(pos, chunk_gen::build(&pos));
		}
		&self.chunks[&pos]
	}
	fn get_active_ch(&self, pos: Vec2<i32>) -> Option<&ChunkContent> {
		self.chunks.get(&pos)
	}
	fn get_mut_ch(&mut self, pos: Vec2<i32>) -> &mut ChunkContent {
		if !self.chunks.contains_key(&pos) {
			self.chunks.insert(pos, chunk_gen::build(&pos));
		}
		self.chunks.get_mut(&pos).unwrap()
	}
	fn set_ch_block(&mut self, chunk_pos: &Vec2<i32>, block_pos: &Vec2<usize>, block_id: usize) {
		self.get_mut_ch(*chunk_pos).blocks[block_pos.x][block_pos.y].set_block(block_id);
	}
	pub fn set_block(&mut self, pos: &Vec2<i32>, block_id : usize) {
		let fpos : Vec2<f64> = pos.as_();
		let chunk_pos : Vec2<i32> = (fpos / Vec2::broadcast(CHUNK_SIZE as f64)).floor().as_();
		let block_pos = {
			pos - chunk_pos * Vec2::broadcast(CHUNK_SIZE as i32)
		};
		let block_pos : Vec2<usize> = block_pos.as_();
		self.set_ch_block(&chunk_pos, &block_pos, block_id)
	}
	pub fn get_block(&mut self, pos: Vec2<i32>) -> &Block {
		let fpos : Vec2<f64> = pos.as_();
		let chunk_pos : Vec2<i32> = (fpos / Vec2::broadcast(CHUNK_SIZE as f64)).floor().as_();
		let block_pos = {
			pos - chunk_pos * Vec2::broadcast(CHUNK_SIZE as i32)
		};
		if block_pos.x < 0 || block_pos.x >= BLOCK_SIZE as i32
		|| block_pos.y < 0 || block_pos.y >= BLOCK_SIZE as i32 {
			println!("-------------------------------------------");
			println!("P {:?}", pos);
			println!("C {:?}", chunk_pos * Vec2::broadcast(CHUNK_SIZE as i32));
			println!("B {:?}", block_pos);
			panic!("Invalid index !!! Contact developper and give him the above values.");
		}
		let block_pos : Vec2<usize> = block_pos.as_();
		&self.get_ch(chunk_pos).blocks[block_pos.x][block_pos.y]
	}
	pub fn get_active_block(&self, pos: Vec2<i32>) -> Option<&Block> {
		let fpos: Vec2<f64> = pos.as_();
		let chunk_pos: Vec2<i32> = (fpos / CHUNK_SIZE as f64).floor().as_();
		let block_pos = {
			pos - chunk_pos * Vec2::broadcast(CHUNK_SIZE as i32)
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
		match &self.get_active_ch(chunk_pos) {
			Some(chunk) => {
				let blc = &chunk.blocks[block_pos.x][block_pos.y];
				// if pos.y == -15 || pos.y == -16 || pos.y == -17 {
				// 	println!("pos {:?}, ch {:?}, bl {:?}, {:?}", pos.y, chunk_pos.y, block_pos.y, blc.block.is_solid);
				// }
				Some(blc)
			},
			None => {
			println!("-------------------------------------------");
			println!("P {:?}", pos);
			println!("C {:?}", chunk_pos * Vec2::broadcast(CHUNK_SIZE as i32));
			println!("B {:?}", block_pos);
			println!("-------------------------------------------");
				None},
		}
	}
	pub fn disp(&mut self, canvas : &mut Canvas<Window>, camera : &Vec2<f64>) {
		let window_size = {
			let (width, height) = canvas.output_size().unwrap();
			Vec2::new(width, height)
		};
		let chunk_scale = BLOCK_SIZE * CHUNK_SIZE;
		let ch_start : Vec2<i32> = {
			// The first chunk is at the coordinates of the camera minus half of the size of the
			// window. All the computation is made in terms of chunk coordinates (instead of block
			// coordinates or pixel coordinates) since it decides which first chunks displays.
			let half_size = window_size.as_() / 2. / chunk_scale as f64;
			((camera / CHUNK_SIZE as f64) - half_size).floor().as_()
		};
		let ch_end : Vec2<i32> = {
			let half_size = window_size.as_() / 2. / chunk_scale as f64;
			((camera / CHUNK_SIZE as f64) + half_size).ceil().as_()
		};
		for chunk_x in ch_start.x..ch_end.x {
			for chunk_y in ch_start.y..ch_end.y {
				let ch_pos = Vec2::new(chunk_x, chunk_y);
				self.get_ch(ch_pos).disp(canvas, ch_pos, camera, 0);
			}
		}
		for chunk_x in ch_start.x..ch_end.x {
			for chunk_y in ch_start.y..ch_end.y {
				let ch_pos = Vec2::new(chunk_x, chunk_y);
				self.get_ch(ch_pos).disp(canvas, ch_pos, camera, 1);
			}
		}
	}
	pub fn place(&mut self, id: usize, pos: Vec2<i32>) {
		let chunk_pos : Vec2<i32> = pos / Vec2::broadcast(CHUNK_SIZE as i32);
		let block_pos: Vec2<usize> = {
			let pos: Vec2<usize> = pos.as_(); let chunk_pos: Vec2<usize> = chunk_pos.as_();
			pos - chunk_pos * Vec2::broadcast(CHUNK_SIZE as usize)
		};
		self.set_ch_block(&chunk_pos, &block_pos, id);
	}
	pub fn update_active(&mut self) {
		let mut active_entities: Vec<(u64,Vec2<i32>)> = Vec::new();
		for tmp in self.entities.iter() {
			active_entities.push((*tmp.0, *tmp.1))
		}
		
		let time = {
			let t = self.last_update.elapsed().as_secs_f64();
			//if t < 1. {return}
			t//10.
		};
		self.last_update = Instant::now();
		if time > 1. {
			println!("skip");
			return;
		}
		let mut all_tasks = Vec::new();
		for (uid, chunk_pos) in active_entities {
			all_tasks.push((
				self.get_active_ch(chunk_pos).unwrap().entities[&uid].update(&self, time),
				uid,
				chunk_pos
			));
		}
		for (entity_tasks, uid, chunk_pos) in all_tasks {
			for task in entity_tasks {
				match task {
					Task::Move(body) => {
						let chunk_coords = (body.pos / CHUNK_SIZE as f64).floor().as_();
						if chunk_pos != chunk_coords {
							let entity = self.get_mut_ch(chunk_pos)
							             .entities.remove(&uid).unwrap();
							self.get_mut_ch(chunk_coords).entities.insert(uid, entity);
							self.entities.insert(uid, chunk_coords);
						} else {
							self.get_mut_ch(chunk_pos)
								.entities.get_mut(&uid).unwrap()
								.move_body(body);
						}
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

	pub fn set_interacter(&mut self, uid: u64) {
		self.interaction_subject_uid = uid;
	}
	pub fn interact(&mut self, interaction: Interaction, start: bool) {
		let action = match interaction {
			Interaction::Up => {Action::MoveUp},
			Interaction::Down => {Action::MoveDown},
			Interaction::Left => {Action::MoveLeft},
			Interaction::Right => {Action::MoveRight},
			Interaction::AimAt(b_pos) => {
				let block = self.get_block(b_pos);
				Action::Aim(
					if block.block.id == 0 
					{Aiming::Position(b_pos)}
					else {Aiming::Block(b_pos)})
			},
			Interaction::Attack => {Action::Attack},
			Interaction::Use => {Action::Use},
		};
		let uid = self.interaction_subject_uid;
		let c: Option<&Vec2<i32>> = self.entities.get(&uid);
		match c {
			Some(chunk_coords) => {
				let coords: Vec2<i32> = *chunk_coords;
				self.get_mut_ch(coords)
					.entities.get_mut(&uid).unwrap()
					.control(action, start)
			},
			None => {println!("No entity bound to key !");},
		}
	}
}
