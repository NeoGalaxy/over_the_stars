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

use terrain::{
	map::{
		Map,
		Interaction
	}
};
use entities::{player::Player};

// #![allow(unused)]
fn main() {
	let mut key_mapping : HashMap<Keycode, Interaction> =
	[(Keycode::Z, Interaction::Up),
	 (Keycode::S, Interaction::Down),
	 (Keycode::Q, Interaction::Left),
	 (Keycode::D, Interaction::Right)]
	 .iter().cloned().collect();
	let mut action_active : [bool; 4] = [false, false, false, false];

	let mut map = Map::new();
	let player_id = Player::create(Vec2::new(0.,-15.0), &mut map);
	map.set_interacter(player_id);
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
	{
		//let pos = map.see_entity(player_id).get_pos();
		map.disp(&mut canvas, &pos);
	}
	canvas.present();
	let mut event_pump = sdl_context.event_pump().unwrap();
	let mut last = Instant::now();
	let mut intervall;
	//let mut cnt = 0;
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => {
					print!("\u{001b}[0K");
					std::io::stdout().flush().expect("Hmmm......");
					break 'running
				},
				Event::KeyDown { keycode: Some(code), .. } => {
					match code {
						Keycode::Up => {action_active[0] = true;}
						Keycode::Down => {action_active[1] = true;}
						Keycode::Left => {action_active[2] = true;}
						Keycode::Right => {action_active[3] = true;},
						_ => {
							if let Some(interaction) = key_mapping.get_mut(&code) {
								map.interact(*interaction, true);
							}
						}
					}
				},
				Event::KeyUp { keycode: Some(code), .. } => {
					match code {
						Keycode::Up => {action_active[0] = false;}
						Keycode::Down => {action_active[1] = false;}
						Keycode::Left => {action_active[2] = false;}
						Keycode::Right => {action_active[3] = false;},
						_ => {
							if let Some(interaction) = key_mapping.get_mut(&code) {
								map.interact(*interaction, false);
							}
						}
					}
				},
				_ => {}
			}
		}
		intervall = last.elapsed().as_secs_f64();
		last = Instant::now();		
		let speed = 15.;
		if action_active[0] {
			pos.y -= speed * intervall;
		}
		if action_active[1] {
			pos.y += speed * intervall;
		}
		if action_active[2] {
			pos.x -= speed * intervall;
		}
		if action_active[3] {
			pos.x += speed * intervall;
		}
		map.update_active();
		canvas.set_draw_color(Color::RGB(0, 255, 255));
		canvas.clear();
		{
			//let pos = map.see_entity(player_id).get_pos();
			map.disp(&mut canvas, &pos);
		}
		canvas.present();
	}
}
