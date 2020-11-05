mod health;

use std::vec::Vec;

use sdl2::{
	render::{RenderTarget,Canvas},
	pixels::Color,
	rect::{Rect,Point},
};
use health::Health;
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

#[allow(dead_code)]
#[derive(Debug)]
enum Aiming {
	Nothing,
	Block(Vec2<i32>),
}
use Aiming::*;

#[derive(Debug)]
pub enum Task {
	Move(Vec2<f64>),
	Break(Vec2<i32>),
	Place(Vec2<i32>, usize),
}

pub trait Entity<T: RenderTarget> {
	fn r#move(&mut self, coords: Vec2<f64>);
	fn get_pos(&self) -> Vec2<f64>;
	fn overlaps(&self, area: vRect<f64, f64>) -> bool;
	fn disp(&self, canvas : &mut Canvas<T>, camera : &Vec2<f64>);
	fn update(&self, map: &Map<T>) -> Vec<Task>;
	//fn activate(&mut self);
	//fn disactivate(&mut self);
}

#[derive(Debug)]
pub struct Player {
	//uid: Option<u64>,
	pos: Vec2<f64>,
	size: Vec2<u32>,
	aim: Aiming,
	inv: Inventory,
	health: Health
}

impl<T: RenderTarget> Entity<T> for Player {
	fn r#move(&mut self, coords: Vec2<f64>) {
		self.pos = coords
	}
	fn get_pos(&self) -> Vec2<f64> {self.pos}
	fn overlaps(&self, area: vRect<f64, f64>) -> bool {
		vRect::new(self.pos.x, self.pos.y, self.size.x as f64, self.size.y as f64)
			.collides_with_rect(area)
	}
	fn disp(&self, canvas : &mut Canvas<T>, camera : &Vec2<f64>) {
		let center = {
			let vect_center : Vec2<i32> = ((self.pos - camera) / BLOCK_SIZE as f64).as_();
			Point::new(vect_center.x, vect_center.y)
		};
		canvas.set_draw_color(Color::RGBA(255,255,255,150));
		canvas.fill_rect(Rect::from_center(center, self.size.x, self.size.y)).unwrap();
	}
	fn update(&self, map: &Map<T>) -> Vec<Task>{
		unimplemented!()
	}
}

impl Player {
	pub fn create<T: RenderTarget>(pos: Vec2<f64>, map: &mut Map<T>) -> u64 {
		map.add_entity(Player {
			pos,
			size: Vec2::new(BLOCK_SIZE - 1, BLOCK_SIZE * 3 - 6),
			aim: Nothing,
			inv: Inventory{},
			health: Health{},
		})
	}
}
