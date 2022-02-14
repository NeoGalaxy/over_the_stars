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
		self.body = body;
	}
	fn get_pos(&self) -> Vec2<f64> {self.body.pos}
	fn overlaps(&self, area: vRect<f64, f64>) -> bool {
		vRect::new(self.body.pos.x, self.body.pos.y, self.size.x as f64, self.size.y as f64)
			.collides_with_rect(area)
	}
	fn tick(&self, map: &Map) -> Vec<Task>{
		let mut tasks = Vec::new();
		let mut new_body = self.body.copy();
		if self.actions.left {new_body.move_at(Vec2::new(-15., 0.), 1./32.);}
		if self.actions.right {new_body.move_at(Vec2::new(15., 0.), 1./32.);}
		if self.actions.up && self.body.on_floor {
			new_body.speed = Vec2::new(0., -30.);
		}

		new_body.accelerate(1./32.);
		new_body.r#move(1./32.);

		let x_h_size = if new_body.pos.x < self.body.pos.x {- self.size.x} else {self.size.x} / 2.; 
		let y_h_size = if new_body.pos.y < self.body.pos.y {- self.size.y} else {self.size.y} / 2.; 

		let x_position = (new_body.pos.x + x_h_size).floor() as i32;
		for y_pos in iter_around(self.body.pos.y, self.size.y) {
			if map.get_active_block(Vec2::new(x_position, y_pos)).unwrap().block.is_solid {
				new_body.pos.x = x_position as f64 - x_h_size;
				new_body.speed.x = 0.0;
				if x_h_size < 0.0 {
					new_body.pos.x += 1.0;
				}
			}
		}


		new_body.on_floor = false;
		let y_position = (new_body.pos.y + y_h_size).floor() as i32;
		for x_pos in iter_around(self.body.pos.x, self.size.x) {
			if map.get_active_block(Vec2::new(x_pos, y_position)).unwrap().block.is_solid {
				new_body.pos.y = y_position as f64 - y_h_size;
				new_body.speed.y = 0.0;
				if y_h_size < 0.0 {
					new_body.pos.y += 1.0;
				} else {
					new_body.on_floor = true;
				}
			}
		}

		std::io::stdout().flush().expect("Hmmm......");

		tasks.push(Task::Move(new_body));
		if self.actions.attack {
			match self.aim {
				Block(pos) => tasks.push(Task::Break(pos)),
				_ => ()
			}
		}
		if self.actions.using {
			match self.aim {
				Position(pos) => tasks.push(Task::Place(pos, 1)),
				_ => ()
			}
		}
		tasks
	}
	fn control(&mut self, action: Action, start: bool) {
		use Action::*;
		match action {
			MoveUp => {self.actions.up = start}
			MoveDown => {self.actions.down = start}
			MoveLeft => {self.actions.left = start}
			MoveRight => {self.actions.right = start}
			Aim(aim) => {
				self.aim = match aim {
					Nothing => Nothing,
					Position(pos) | Block(pos) => {
						let xdiff = (self.body.pos.x - 0.5) as i32 - pos.x;
						let ydiff = (self.body.pos.y - 0.5) as i32 - pos.y;
						if xdiff.abs() + ydiff.abs() < 10 {
							aim
						} else {
							Nothing
						}
					},
				}

			}
			Attack => {self.actions.attack = start}
			Use => {self.actions.using = start}
		}
	}
	fn get_light(&mut self) -> i32 {
		return 30;
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

		match self.aim {
			Nothing => (),
			Position(pos) | Block(pos) => {
				let pix_pos = ((pos.as_::<f64>() - camera) * BLOCK_SIZE as f64).as_()
					+ screen_center;
				canvas.set_draw_color(Color::RGBA(255, 255, 255, 80));
				canvas.draw_rect(Rect::new(pix_pos.x, pix_pos.y, BLOCK_SIZE, BLOCK_SIZE))
				.unwrap();
				canvas.draw_rect(Rect::new(pix_pos.x+1, pix_pos.y+1, BLOCK_SIZE-2, BLOCK_SIZE-2))
				.unwrap();
			}
		}
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
			body: Body{
				pos,
				speed: Vec2::new(0.,0.),
				acceleration: Vec2::new(0.,60.),
				on_floor: false
			},
			size: Vec2::new(1.3, 2.8),
			aim: Nothing,
			inv: Inventory{},
			health: Health{},
			actions: Default::default(),
		}))
	}
}
