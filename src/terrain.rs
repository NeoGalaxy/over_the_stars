use sdl2::video::Window;
use sdl2::surface::Surface;

use sdl2::render::Canvas;
use vek::Vec2;

pub mod blocks;
pub mod map;

pub trait Displayable {
	fn disp(&self, canvas : &mut Canvas<Window>, camera : &Vec2<f64>);
	fn disp_surf(&self, canvas : &mut Canvas<Surface>, camera : &Vec2<f64>);
}