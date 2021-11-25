use sdl2::surface::Surface;
use sdl2::video::Window;
use crate::terrain::Displayable;
use std::{
	vec::Vec,
	boxed::Box,
	io::Write,
};

use sdl2::{
	render::{Canvas},
	pixels::Color,
	rect::{Rect,Point},
};
use super::{
	Body,
	Entity,
	Task,
	health::Health,
	Action,
	Aiming::{self, *},
};
use crate::{
	inventory::Inventory,
	terrain::{
		map::Map,
		blocks::{
			BLOCK_SIZE
		},
	},
};

use vek::{
	vec::repr_c::vec2::Vec2,
	geom::repr_c::Rect as vRect,
};

#[derive(Debug, Default)]
struct PlayerActionList {
	up: bool,
	down: bool,
	left: bool,
	right: bool,
	attack: bool,
	using: bool,
}

#[derive(Debug)]
pub struct Player {
	//uid: Option<u64>,
	body: Body,
	size: Vec2<f64>,
	aim: Aiming,
	inv: Inventory,
	health: Health,
	actions: PlayerActionList
}

fn compute_start(from_pos: Vec2<f64>, to_pos: Vec2<f64>, half_size: f64) -> i32 {
	let downwards = from_pos.y < to_pos.y;
	return (
		if downwards {(from_pos.y + half_size).ceil()}
		else         {(from_pos.y - half_size).ceil()}
	) as i32
}

fn compute_end(from_pos: Vec2<f64>, to_pos: Vec2<f64>, half_size: f64) -> i32 {
	let downwards = from_pos.y < to_pos.y;
	return (
		if downwards {(to_pos.y + half_size).floor()} 
		else         {(to_pos.y - half_size).floor()}
	) as i32
}

impl Entity for Player {
	fn move_body(&mut self, body: Body) {
		self.body = body
	}
	fn get_pos(&self) -> Vec2<f64> {self.body.pos}
	fn overlaps(&self, area: vRect<f64, f64>) -> bool {
		vRect::new(self.body.pos.x, self.body.pos.y, self.size.x as f64, self.size.y as f64)
			.collides_with_rect(area)
	}
	fn update(&self, map: &Map, time: f64) -> Vec<Task>{
		let mut tasks = Vec::new();
		let mut new_body = self.body.copy();
		{
			if self.actions.left {new_body.move_at(Vec2::new(-15., 0.), time);}
			if self.actions.right {new_body.move_at(Vec2::new(15., 0.), time);}
			if self.actions.up {
				new_body.speed = Vec2::new(0., -15.);
			}
		}
		new_body.accelerate(time);
		new_body.r#move(time);
		let left = self.body.pos.x - (self.size.x as f64) / 2.;
		let downwards = self.body.pos.y < new_body.pos.y;
		///////////
		let start = compute_start(self.body.pos, new_body.pos, self.size.y / 2.);
		let end = compute_end(self.body.pos, new_body.pos, self.size.y / 2.);
		let iterator: Box<dyn Iterator<Item = i32>> = {
			if downwards {Box::new(start..end+1)} else { Box::new((end..start+1).rev()) }
		};
		///////////
		for y in iterator {
			let can_move_here = 
			(left.floor() as i32..(left as f64 + self.size.x as f64/2.).ceil() as i32).all(
				|x| !map.get_active_block(Vec2::new(x,y)).unwrap().block.is_solid
			);
			if !can_move_here {
				new_body.speed.y = 0.;
				let dir_size = (if downwards {-1.} else {1.}) * self.size.y;
				new_body.pos.y = dir_size/2. + (y as i32) as f64;
				if !downwards {
					new_body.pos.y += 1.;
				}
				break;
			}
		}
		std::io::stdout().flush().expect("Hmmm......");
				
		tasks.push(Task::Move(new_body));
		tasks
	}
	fn control(&mut self, action: Action, start: bool) {
		use Action::*;
		match action {
			MoveUp => {self.actions.up = start}
			MoveDown => {self.actions.down = start}
			MoveLeft => {self.actions.left = start}
			MoveRight => {self.actions.right = start}
			Aim(aim) => {self.aim = aim}
			Attack => {self.actions.attack = start}
			Use => {self.actions.using = start}
		}
	}
}

impl Displayable for Player {
	fn disp(&self, canvas : &mut Canvas<Window>, camera : &Vec2<f64>) {
			let (width, height) = canvas.output_size().unwrap();
			let screen_center = Vec2::new(width as i32, height as i32)/2;
		let center = {
			let vect_center = ((self.body.pos - camera) * BLOCK_SIZE as f64).as_() + screen_center;
			Point::new(vect_center.x, vect_center.y)
		};

		canvas.set_draw_color(Color::RGBA(255,255,255,150));
		let px_size = (self.size * BLOCK_SIZE as f64).as_();
		canvas.fill_rect(Rect::from_center(center, px_size.x, px_size.y)).unwrap();
	}
	fn disp_surf(&self, canvas : &mut Canvas<Surface>, camera : &Vec2<f64>) {
			let (width, height) = canvas.output_size().unwrap();
			let screen_center = Vec2::new(width as i32, height as i32)/2;
		let center = {
			let vect_center = ((self.body.pos - camera) * BLOCK_SIZE as f64).as_() + screen_center;
			Point::new(vect_center.x, vect_center.y)
		};

		canvas.set_draw_color(Color::RGBA(255,255,255,150));
		let px_size = (self.size * BLOCK_SIZE as f64).as_();
		canvas.fill_rect(Rect::from_center(center, px_size.x, px_size.y)).unwrap();
	}
}

impl Player {
	pub fn create(pos: Vec2<f64>, map: &mut Map) -> u64 {
		map.add_entity(Box::new(Player {
			body: Body{pos, speed: Vec2::new(0.,0.), acceleration: Vec2::new(0.,30.)},
			size: Vec2::new(0.95, 30./16.),
			aim: Nothing,
			inv: Inventory{},
			health: Health{},
			actions: Default::default(),
		}))
	}
}
