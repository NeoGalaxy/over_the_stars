extern crate sdl2; 
pub mod terrain;
pub mod inventory;
pub mod entities;

use std::collections::HashMap;
use std::io::Write;
use sdl2::{
	pixels::Color,
	event::Event,
	keyboard::Keycode,
};
use sdl2_sys::SDL_WindowFlags;
use vek::vec::repr_c::vec2::Vec2;

use std::time::{Instant};

use terrain::{map::Map};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
	Up,
	Down,
	Left,
	Right,
}

//#![allow(unused)]
fn main() {
	let mut map = Map::new();
	let mut key_mapping : HashMap<Keycode, Action> =
	[(Keycode::Up, Action::Up),
	 (Keycode::Down, Action::Down),
	 (Keycode::Left, Action::Left),
	 (Keycode::Right, Action::Right),
	 (Keycode::Z, Action::Up),
	 (Keycode::S, Action::Down),
	 (Keycode::Q, Action::Left),
	 (Keycode::D, Action::Right)]
	 .iter().cloned().collect();
	let mut action_active : HashMap<Action, bool> =
	[(Action::Up, false),
	 (Action::Down, false),
	 (Action::Left, false),
	 (Action::Right, false)]
	 .iter().cloned().collect();
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();
 
	let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
		.set_window_flags(SDL_WindowFlags::SDL_WINDOW_RESIZABLE as u32)
		.position_centered()
		.build()
		.unwrap();
	let mut canvas = window.into_canvas().present_vsync().build().unwrap();
	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	let mut pos = Vec2::new(0.,0.);
	map.disp(&mut canvas, &pos);
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut last = Instant::now();
	let mut intervall;
	let mut cnt = 0;
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					print!("\u{001b}[0K");
					std::io::stdout().flush().expect("Hmmm......");
					break 'running
				},
				Event::KeyDown { keycode: Some(code), .. } => {
					if let Some(action) = key_mapping.get_mut(&code) {
						if !action_active.get(action).unwrap() {
							*action_active.get_mut(action).unwrap() = true;
						}
					}
				},
				Event::KeyUp { keycode: Some(code), .. } => {
					if let Some(action) = key_mapping.get_mut(&code) {
						if *action_active.get(action).unwrap() {
							*action_active.get_mut(action).unwrap() = false;
						}
					}
				},
				_ => {}
			}
		}
		intervall = last.elapsed().as_secs_f64();
		last = Instant::now();
		cnt += 1;
		if cnt >= 5 {
			cnt = 0;
		
			print!("\u{001b}[0K{}\t{:?}\r", 1./intervall, pos);
			std::io::stdout().flush().expect("Hmmm......");
		}
		let speed = 15.;
		if action_active[&Action::Up] {
			pos.y -= speed * intervall;
		}
		if action_active[&Action::Down] {
			pos.y += speed * intervall;
		}
		if action_active[&Action::Left] {
			pos.x -= speed * intervall;
		}
		if action_active[&Action::Right] {
			pos.x += speed * intervall;
		}
		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		map.disp(&mut canvas, &pos);
		canvas.present();
	}
}
