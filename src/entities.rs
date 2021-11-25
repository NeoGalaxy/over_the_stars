mod health;
pub mod player;

use crate::terrain::Displayable;
use std::{
	vec::Vec,
};

use crate::{
	terrain::{
		map::Map,
	},
};

use vek::{
	vec::repr_c::vec2::Vec2,
	geom::repr_c::Rect as vRect,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum Aiming {
	Nothing,
	Block(Vec2<i32>),
}

#[derive(Debug)]
pub struct Body {
	pub pos: Vec2<f64>,
	pub speed: Vec2<f64>,
	pub acceleration: Vec2<f64>,
}

impl Body {
	pub fn copy(& self) -> Body{
		Body {
			..*self
		}
	}
	pub fn accelerate(&mut self, time: f64) -> &mut Body {
		self.speed = self.speed + self.acceleration * time;
		return self
	}
	pub fn accelerate_at(&mut self, acceleration: Vec2<f64>, time: f64) -> &mut Body {
		self.speed = self.speed + acceleration * time;
		return self
	}
	pub fn r#move(&mut self, time: f64) -> &mut Body {
		self.pos = self.pos + self.speed * time;
		return self
	}
	pub fn move_at(&mut self, speed: Vec2<f64>, time: f64) -> &mut Body {
		self.pos = self.pos + speed * time;
		return self
	}
	
}

#[derive(Debug)]
pub enum Task {
	Move(Body),
	Break(Vec2<i32>),
	Place(Vec2<i32>, usize),
}

#[derive(Debug)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

#[derive(Debug)]
pub enum Action {
	MoveUp,
	MoveDown,
	MoveLeft,
	MoveRight,
	//MoveTowards(Vec2<f64>),
	Aim(Aiming),
	Attack,
	Use,
}

pub trait Entity: Displayable {
	fn move_body(&mut self, body: Body);
	fn get_pos(&self) -> Vec2<f64>;
	fn overlaps(&self, area: vRect<f64, f64>) -> bool;
	fn update(&self, map: &Map, time: f64) -> Vec<Task>;
	fn control(&mut self, action: Action, start: bool);
	//fn activate(&mut self);
	//fn disactivate(&mut self);
}
