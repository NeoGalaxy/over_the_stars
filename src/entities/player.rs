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

fn iter_around(center: f64, dist: f64) -> std::ops::Range<i32> {
	return (center - dist/2.).floor() as i32..(center + dist/2.).ceil() as i32
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
				//new_body.speed = Vec2::new(0., -15.);
				new_body.move_at(Vec2::new(0., -15.), time);
			}
			if self.actions.down {
				new_body.move_at(Vec2::new(0., 15.), time);
			}
		}

		let mut to_visit: Vec<Vec2<i32>> = Vec::with_capacity(10);
		//new_body.accelerate(time);
		new_body.r#move(time);

		let x_h_size = if new_body.pos.x < self.body.pos.x {- self.size.x} else {self.size.x} / 2.; 
		let y_h_size = if new_body.pos.y < self.body.pos.y {- self.size.y} else {self.size.y} / 2.; 

		if new_body.pos.x != self.body.pos.x {
			for y in iter_around(new_body.pos.y, self.size.y) {
				to_visit.push(Vec2::new((new_body.pos.x + x_h_size).floor() as i32, y));
			}
		}
		if new_body.pos.y != self.body.pos.y {
			for x in iter_around(new_body.pos.x, self.size.x) {
				to_visit.push(Vec2::new(x, (new_body.pos.y + y_h_size).floor() as i32));
			}
		}

		let mut x_prog_min: f64 = 1.;
		let mut y_prog_min: f64 = 1.;
		for mut pos in to_visit {
			if map.get_active_block(pos).unwrap().block.is_solid {
				if new_body.pos.x < self.body.pos.x {
					pos.x += 1;
				}
				if new_body.pos.y < self.body.pos.y {
					pos.y += 1;
				}
				if new_body.pos.x - self.body.pos.x != 0.0 {
					let x_progress = (pos.x as f64 - self.body.pos.x -  x_h_size) / (new_body.pos.x - self.body.pos.x);
					if x_progress < x_prog_min {
						x_prog_min = x_progress;
					}
				}

				if new_body.pos.y - self.body.pos.y != 0.0 {
					let y_progress = (pos.y as f64 - self.body.pos.y - y_h_size) / (new_body.pos.y - self.body.pos.y);
					if y_progress < y_prog_min {
						y_prog_min = y_progress;
					}
				}
			}
		}
		if x_prog_min < 1. {
			new_body.pos.x = self.body.pos.x + (new_body.pos.x - self.body.pos.x) * x_prog_min;
			new_body.speed.x = 0.0;
		}
		if y_prog_min < 1. {
			new_body.pos.y = self.body.pos.y + (new_body.pos.y - self.body.pos.y) * y_prog_min;
			new_body.speed.y = 0.0;
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
